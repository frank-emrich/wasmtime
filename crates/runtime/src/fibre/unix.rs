//! The unix fiber implementation has some platform-specific details
//! (naturally) but there's a few details of the stack layout which are common
//! amongst all platforms using this file. Remember that none of this applies to
//! Windows, which is entirely separate.
//!
//! The stack is expected to look pretty standard with a guard page at the end.
//! Currently allocation happens in this file but this is probably going to be
//! refactored to happen somewhere else. Otherwise though the stack layout is
//! expected to look like so:
//!
//!
//! ```text
//! 0xB000 +-----------------------+   <- top of stack
//!        | unused                |
//! 0xAff8 +-----------------------+
//!        | *const u8             |   <- last sp to resume from
//! 0xAff0 +-----------------------+   <- 16-byte aligned
//!        |                       |
//!        ~        ...            ~   <- actual native stack space to use
//!        |                       |
//! 0x1000 +-----------------------+
//!        |  guard page           |
//! 0x0000 +-----------------------+
//! ```
//!
//! Here `0xAff8` is filled in temporarily while `resume` is running. The fiber
//! started with 0xB000 as a parameter so it knows how to find this.
//! Additionally `resumes` stores state at 0xAff0 to restart execution, and
//! `suspend`, which has 0xB000 so it can find this, will read that and write
//! its own resumption information into this slot as well.

#![allow(unused_macros)]

use std::alloc::{alloc, dealloc, Layout};
use std::io;
use std::ops::Range;
use std::ptr;
use wasmtime_continuations::{SwitchDirection, SwitchDirectionEnum};

use crate::{VMContext, VMFuncRef, VMOpaqueContext, ValRaw};

#[derive(Debug)]
pub struct FiberStack {
    // The top of the stack; for stacks allocated by the fiber implementation itself,
    // the base address of the allocation will be `top.sub(len.unwrap())`
    top: *mut u8,
    // The length of the stack
    len: usize,
    // whether or not this stack was mmap'd
    mmap: bool,
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
                mmap: true,
            })
        }
    }

    pub fn malloc(size: usize) -> io::Result<Self> {
        unsafe {
            let layout = Layout::array::<u8>(size).unwrap();
            let base = alloc(layout);
            FiberStack::from_raw_parts(base, size)
        }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn from_raw_parts(base: *mut u8, len: usize) -> io::Result<Self> {
        Ok(Self {
            top: base.add(len),
            len,
            mmap: false,
        })
    }

    pub fn top(&self) -> Option<*mut u8> {
        Some(self.top)
    }

    pub fn range(&self) -> Option<Range<usize>> {
        let base = unsafe { self.top.sub(self.len) as usize };
        Some(base..base + self.len)
    }
}

impl Drop for FiberStack {
    fn drop(&mut self) {
        unsafe {
            if self.mmap {
                let ret = rustix::mm::munmap(self.top.sub(self.len) as _, self.len);
                debug_assert!(ret.is_ok());
            } else {
                let layout = Layout::array::<u8>(self.len).unwrap();
                dealloc(self.top.sub(self.len), layout);
            }
        }
    }
}

pub struct Fiber;

pub struct Suspend(*mut u8);

extern "C" {
    fn wasmtime_fibre_init(
        top_of_stack: *mut u8,
        entry_point: extern "C" fn(*mut u8, *mut VMContext, SwitchDirection, *mut VMFuncRef),
        func_ref: *mut VMFuncRef,
    );
    fn wasmtime_fibre_switch(
        top_of_stack: *mut u8,
        active_vmctx: *mut VMContext,
        direction: SwitchDirection,
    ) -> SwitchDirection;
    #[allow(dead_code)] // only used in inline assembly for some platforms
    fn wasmtime_fibre_start();
}

/// This function is what is actually executed within a continuation's Fiber.
/// It is only ever called from `wasmtime_fibre_start`.
/// This function should not return directly, but instead switches back to the
/// parent with an appropriate `SwitchDirection` value once the execution of the
/// function denoted in `func_ref` returns.
/// This function must not panic, instead returning from it can be used to
/// signal an error: `wasmtime_fibre_start` deliberately executes an illegal
/// instruction if `fiber_start` returns.
extern "C" fn fiber_start(
    top_of_stack: *mut u8,
    caller: *mut VMContext,
    switch_direction: SwitchDirection,
    func_ref: *mut VMFuncRef,
) {
    if cfg!(debug_assertions) && switch_direction.discriminant != SwitchDirectionEnum::Resume {
        // This if block basically acts as an assertion: If `switch_direction`
        // is not a `Resume`, something is terribly wrong.
        return;
    }

    let args_capacity = switch_direction.data0;
    let args_ptr = switch_direction.data1 as *mut ValRaw;

    unsafe {
        let array_trampoline = (*func_ref).array_call;
        let callee = (*func_ref).vmctx;
        let caller_opaque = VMOpaqueContext::from_vmcontext(caller);
        array_trampoline(callee, caller_opaque, args_ptr, args_capacity as usize);

        // We "return" by switching back to the parent
        let return_ = SwitchDirection::return_();
        Suspend(top_of_stack).switch(caller, return_);
    }
}

impl Fiber {
    pub fn new(stack: &FiberStack, func_ref: *mut VMFuncRef) -> io::Result<Self> {
        unsafe {
            wasmtime_fibre_init(stack.top, fiber_start, func_ref);
        }

        Ok(Self)
    }

    pub(crate) fn resume(
        &self,
        stack: &FiberStack,
        active_vmctx: *mut VMContext,
        args_ptr: *mut u8,
        args_capacity: u32,
    ) -> SwitchDirection {
        unsafe {
            let payload = SwitchDirection::resume(args_ptr, args_capacity);
            SwitchDirection::from(wasmtime_fibre_switch(stack.top, active_vmctx, payload))
        }
    }
}

impl Suspend {
    pub fn switch(&self, active_vmcontext: *mut VMContext, payload: SwitchDirection) {
        // `payload::discrimination` should be either
        // `SwitchDirectionEnum::Suspend` or `SwitchDirectionEnum::Return`, but
        // we cannot check that and panic in this function.
        unsafe {
            wasmtime_fibre_switch(self.0, active_vmcontext, payload);
        }
    }

    pub fn from_top_ptr(ptr: *mut u8) -> Self {
        Suspend(ptr)
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
    } else {
        compile_error!("fibers are not supported on this CPU architecture");
    }
}
