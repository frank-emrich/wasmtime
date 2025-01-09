// Stack switching feature fiber stack allocators
//
// * on_demand: allocates memory lazily
// * pooling: preallocates a chunk of memory eagerly
//

use crate::prelude::*;
use anyhow::Result;
use wasmtime_environ::stack_switching::StackSwitchingConfig;

pub use crate::runtime::vm::continuation::imp::{FiberStack, VMContRef};

// This module is dead code if the pooling allocator is toggled.
#[allow(dead_code)]
pub mod on_demand {
    use super::*;

    #[derive(Debug)]
    pub struct InnerAllocator {
        stack_size: usize,
    }

    impl InnerAllocator {
        pub fn new(config: &StackSwitchingConfig) -> Result<Self> {
            Ok(InnerAllocator {
                stack_size: config.stack_size,
            })
        }

        pub fn allocate(&mut self) -> Result<(*mut VMContRef, FiberStack)> {
            let stack = super::FiberStack::new(self.stack_size);
            let stack = stack.map_err(|_| anyhow::anyhow!("Fiber stack allocation failed"));
            let contref = Box::into_raw(Box::new(VMContRef::empty()));
            Ok((contref, stack?))
        }

        pub fn deallocate(&mut self, contref: *mut VMContRef) {
            // In on-demand mode, we actually deallocate the continuation.
            unsafe { core::mem::drop(Box::from_raw(contref)) };
        }
    }
}

use on_demand as imp;

pub struct StacksAllocator {
    inner: imp::InnerAllocator,
}

impl StacksAllocator {
    pub fn new(config: &StackSwitchingConfig) -> Result<Self> {
        Ok(Self {
            inner: imp::InnerAllocator::new(config)?,
        })
    }

    /// Note that for technical reasons, we return the `VMContRef` and
    /// `FiberStack` separately. In particular, the stack field of the
    /// continuation does not correspond to/point to that stack, yet. Instead, the
    /// `VMContRef` returned here has an empty stack (i.e., `None` in the
    /// baseline implementation, or an empty dummy stack in the optimized
    /// implementation).
    /// This allows the baseline implementation of the allocator interface to
    /// initialize a new `Fiber` from the `FiberStack`. then save it in the
    /// `VMContRef`.
    ///
    /// Note that the `revision` counter of the returned `VMContRef` may be
    /// non-zero and must not be decremented.
    pub fn allocate(&mut self) -> Result<(*mut VMContRef, FiberStack)> {
        self.inner.allocate()
    }

    /// This may not actually deallocate the underlying memory, but simply
    /// return the `VMContRef` to a pool.
    pub fn deallocate(&mut self, contref: *mut VMContRef) {
        self.inner.deallocate(contref)
    }
}
