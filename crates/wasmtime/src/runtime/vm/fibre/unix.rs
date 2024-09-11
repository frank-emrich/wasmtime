//! The stack layout is expected to look like so:
//!
//!
//! ```text
//! 0xB000 +-----------------------+   <- top of stack (TOS)
//!        | saved PC              |
//! 0xAff8 +-----------------------+
//!        | saved RBP             |
//! 0xAff0 +-----------------------+
//!        | saved RSP             |
//! 0xAfe8 +-----------------------+   <- beginning of "control context",
//!        | 0                     |
//! 0xAfe0 +-----------------------+   <- beginning of usable stack space
//!        |                       |      (16-byte aligned)
//!        |                       |
//!        ~        ...            ~   <- actual native stack space to use
//!        |                       |
//! 0x1000 +-----------------------+
//!        |  guard page           |   <- (not currently enabled)
//! 0x0000 +-----------------------+
//! ```
//!
//! The "control context" indicates how to resume a computation. The layout is
//! determined by Cranelift's stack_switch instruction, which reads and writes
//! these fields. The fields used as follows, where we distinguish two cases:
//!
//! 1
//! If the continuation is currently active (i.e., running directly, or ancestor
//! of the running continuation), it stores the PC, RSP, and RBP of the *parent* of
//! the running continuation.
//!
//! 2 If the picture shows a suspended computation, then store the PC,
//! RSP, and RBP at the time of the suspension.
//!
//! Note that this design ensures that external tools can construct backtraces
//! in the presence of stack switching by using frame pointers only:
//! The wasmtime_fibre_start trampoline uses the address of the RBP field in the
//! control context (0xAff0) as its frame pointer. This means that when passing
//! the wasmtime_fibre_start frame while doing frame pointer walking, the parent
//! of that frame is the last frame in the parent of this continuation.
//!
//! Wasmtime's own mechanism for constructing backtraces also relies on frame
//! pointer chains. However, it understands continuations and does not rely on
//! the trickery outlined here to go from the frames in one continuation to the
//! parent.

#![allow(unused_macros)]

use std::alloc::{alloc, dealloc, Layout};
use std::io;
use std::ops::Range;
use std::ptr;
use wasmtime_continuations::ControlEffect;

use crate::runtime::vm::{VMContext, VMFuncRef, VMOpaqueContext, ValRaw};

#[derive(Debug,PartialEq, Eq)]
pub enum Allocator {
    Malloc,
    Mmap,
    Custom,
}

#[derive(Debug)]
#[repr(C)]
pub struct FiberStack {
    // The top of the stack; for stacks allocated by the fiber implementation itself,
    // the base address of the allocation will be `top.sub(len.unwrap())`
    top: *mut u8,
    // The length of the stack
    len: usize,
    // allocation strategy
    allocator: Allocator,
}

impl FiberStack {
    pub fn new(size: usize) -> io::Result<Self> {
        // Round up our stack size request to the nearest multiple of the
        // page size.
        let page_size = rustix::param::page_size();
        let size = if size == 0 {
            page_size
        } else {
            (size + (page_size - 1)) & (!(page_size - 1))
        };

        unsafe {
            // Add in one page for a guard page and then ask for some memory.
            let mmap_len = size + page_size;
            let mmap = rustix::mm::mmap_anonymous(
                ptr::null_mut(),
                mmap_len,
                rustix::mm::ProtFlags::empty(),
                rustix::mm::MapFlags::PRIVATE,
            )?;

            rustix::mm::mprotect(
                mmap.cast::<u8>().add(page_size).cast(),
                size,
                rustix::mm::MprotectFlags::READ | rustix::mm::MprotectFlags::WRITE,
            )?;

            Ok(Self {
                top: mmap.cast::<u8>().add(mmap_len),
                len: mmap_len,
                allocator: Allocator::Mmap,
            })
        }
    }

    pub fn malloc(size: usize) -> io::Result<Self> {
        unsafe {
            let layout = Layout::array::<u8>(size).unwrap();
            let base = alloc(layout);
            Ok(Self {
                top: base.add(size),
                len: size,
                allocator: Allocator::Malloc,
            })
        }
    }

    pub fn unallocated() -> Self {
        Self {
            top: std::ptr::null_mut(),
            len: 0,
            allocator: Allocator::Custom,
        }
    }

    pub fn is_unallocated(&self) -> bool {
        debug_assert_eq!(self.len == 0, self.top == std::ptr::null_mut());
        self.len == 0
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn from_raw_parts(base: *mut u8, len: usize) -> io::Result<Self> {
        Ok(Self {
            top: base.add(len),
            len,
            allocator: Allocator::Custom,
        })
    }


    pub fn is_from_raw_parts(&self) -> bool {
        self.allocator == Allocator::Custom
    }

    pub fn top(&self) -> Option<*mut u8> {
        Some(self.top)
    }

    pub fn range(&self) -> Option<Range<usize>> {
        let base = unsafe { self.top.sub(self.len) as usize };
        Some(base..base + self.len)
    }

    pub fn initialize(
        &self,
        func_ref: *const VMFuncRef,
        caller_vmctx: *mut VMContext,
        args_ptr: *mut ValRaw,
        args_capacity: usize,
    ) {
        unsafe {
            wasmtime_fibre_init(
                self.top,
                func_ref,
                caller_vmctx,
                args_ptr,
                args_capacity,
            );
        }
    }

    pub(crate) fn resume(&self) -> ControlEffect {
        unsafe {
            let reason = ControlEffect::resume().into();
            ControlEffect::from(wasmtime_fibre_switch(self.top, reason))
        }
    }

    pub fn suspend(&self, payload: ControlEffect) {
        suspend_fiber(self.top, payload)
    }
}

pub fn suspend_fiber(top_of_stack: *mut u8, payload: ControlEffect) {
    unsafe {
        let arg = payload.into();
        wasmtime_fibre_switch(top_of_stack, arg);
    }
}

impl Drop for FiberStack {
    fn drop(&mut self) {
        unsafe {
            match self.allocator {
                Allocator::Mmap => {
                    let ret = rustix::mm::munmap(self.top.sub(self.len) as _, self.len);
                    debug_assert!(ret.is_ok());
                }
                Allocator::Malloc => {
                    let layout = Layout::array::<u8>(self.len).unwrap();
                    dealloc(self.top.sub(self.len), layout);
                }
                Allocator::Custom => {} // It's the creator's responsibility to reclaim the memory.
            }
        }
    }
}

extern "C" {
    // We allow "improper ctypes" here (i.e., passing values as parameters in an
    // extern C function that Rust deems non FFI-safe): The two problematic
    // parameters, namely `func_ref` and `args_ptr`, are piped through into
    // `fiber_start` (a Rust function), and not accessed in between.
    #[allow(improper_ctypes)]
    fn wasmtime_fibre_init(
        top_of_stack: *mut u8,
        func_ref: *const VMFuncRef,
        caller_vmctx: *mut VMContext,
        args_ptr: *mut ValRaw,
        args_capacity: usize,
    );
    fn wasmtime_fibre_switch(top_of_stack: *mut u8, payload: u64) -> u64;
    #[allow(dead_code)] // only used in inline assembly for some platforms
    fn wasmtime_fibre_start();
}

/// This function is responsible for actually running a wasm function inside a
/// continuation. It is only ever called from `wasmtime_fibre_start`. Hence, it
/// must never return.
extern "C" fn fiber_start(
    top_of_stack: *mut u8,
    func_ref: *const VMFuncRef,
    caller_vmctx: *mut VMContext,
    args_ptr: *mut ValRaw,
    args_capacity: usize,
) {
    unsafe {
        let func_ref = &*func_ref;
        let array_call_trampoline = func_ref.array_call;
        let caller_vmxtx = VMOpaqueContext::from_vmcontext(caller_vmctx);
        let callee_vmxtx = func_ref.vmctx;

        // NOTE(frank-emrich) The usage of the `caller_vmctx` is probably not
        // 100% correct here. Currently, we determine the "caller" vmctx when
        // initilizing the fiber stack/continuation (i.e. as part of
        // `cont.new`). However, we may subsequenly `resume` the continuation
        // from a different Wasm instance. The way to fix this would be to make
        // the currently active `VMContext` an additional parameter of
        // `wasmtime_fibre_switch` and pipe it through to this point. However,
        // since the caller vmctx is only really used to access stuff in the
        // underlying `Store`, it's fine to be slightly sloppy about the exact
        // value we set.
        array_call_trampoline(callee_vmxtx, caller_vmxtx, args_ptr, args_capacity);

        // Switch back to parent, indicating that the continuation returned.
        let reason = ControlEffect::return_();
        suspend_fiber(top_of_stack, reason)
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
    } else {
        compile_error!("fibers are not supported on this CPU architecture");
    }
}
