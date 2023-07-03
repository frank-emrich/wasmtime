//! Continuations TODO

use crate::instance::TopOfStackPointer;
use crate::vmcontext::{VMArrayCallFunction, VMFuncRef, VMOpaqueContext, ValRaw};
use crate::{Instance, TrapReason};
use std::mem;
use wasmtime_environ::MAXIMUM_CONTINUATION_PAYLOAD_COUNT;
use wasmtime_fibre::{Fiber, FiberStack, Suspend};

type ContinuationFiber = Fiber<'static, (), u32, ()>;
type Yield = Suspend<(), u32, ()>;

/// TODO
#[repr(C)]
pub struct ContinuationObject {
    fiber: *mut ContinuationFiber,
    len: usize,
    capacity: usize,
    args: *mut u128,
}

/// M:1 Many-to-one mapping. A single ContinuationObject may be
/// referenced by multiple ContinuationReference, though, only one
/// ContinuationReference may hold a non-null reference to the object
/// at a given time.
#[repr(C)]
pub struct ContinuationReference(Option<*mut ContinuationObject>);

/// TODO
#[inline(always)]
pub fn cont_ref_get_cont_obj(
    contref: *mut ContinuationReference,
) -> Result<*mut ContinuationObject, TrapReason> {
    let contopt = unsafe { contref.as_mut().unwrap().0 };
    match contopt {
        None => Err(TrapReason::user_with_backtrace(anyhow::Error::msg(
            "Continuation is already taken",
        ))), // TODO(dhil): presumably we can set things up such that
        // we always read from a non-null reference.
        Some(contobj) => Ok(contobj as *mut ContinuationObject),
    }
}

/// TODO
#[inline(always)]
pub fn cont_obj_get_args(_instance: &mut Instance, obj: *mut ContinuationObject) -> *mut u128 {
    unsafe { (*obj).args }
}

/// TODO
#[inline(always)]
pub fn cont_obj_get_args_at_next_free(
    instance: &mut Instance,
    obj: *mut ContinuationObject,
    nargs: usize,
) -> *mut u128 {
    let args_ptr = cont_obj_get_args(instance, obj);
    let args_len = unsafe { (*obj).len };
    unsafe { (*obj).len += nargs };
    unsafe { args_ptr.offset(args_len as isize) }
}

/// TODO
#[inline(always)]
pub fn new_cont_ref(contobj: *mut ContinuationObject) -> *mut ContinuationReference {
    let contref = Box::new(ContinuationReference(Some(contobj)));
    Box::into_raw(contref)
}

/// TODO
#[inline(always)]
pub fn drop_cont_obj(contobj: *mut ContinuationObject) {
    let args: Vec<u128> =
        unsafe { Vec::from_raw_parts((*contobj).args, (*contobj).len, (*contobj).capacity) };
    mem::drop(unsafe { (*contobj).fiber });
    mem::drop(args);
    mem::drop(contobj)
}

/// TODO
#[inline(always)]
pub fn ensure_suspend_payloads_capacity(_instance: &mut Instance, _npayloads: usize) {
    todo!()
    // if unsafe { (*contobj).payloads.len() } < npayloads {
    //     Vec::resize(unsafe { &mut (*contobj).payloads }, npayloads, 0u128)
    // }
}

/// TODO
#[inline(always)]
pub fn cont_new(instance: &mut Instance, func: *mut u8, nargs: usize) -> *mut u8 {
    let func = func as *mut VMFuncRef;
    let callee_ctx = unsafe { (*func).vmctx };
    let caller_ctx = VMOpaqueContext::from_vmcontext(instance.vmctx());
    let f = unsafe {
        mem::transmute::<
            VMArrayCallFunction,
            unsafe extern "C" fn(*mut VMOpaqueContext, *mut VMOpaqueContext, *mut ValRaw, usize),
        >((*func).array_call)
    };
    let mut args = Vec::with_capacity(nargs);
    let args_ptr = args.as_mut_ptr();
    let fiber = Box::new(
        Fiber::new(
            FiberStack::new(4096).unwrap(),
            move |_first_val: (), _suspend: &Yield| unsafe {
                f(callee_ctx, caller_ctx, args_ptr as *mut ValRaw, nargs)
            },
        )
        .unwrap(),
    );

    let contobj = Box::new(ContinuationObject {
        fiber: Box::into_raw(fiber),
        len: 0,
        capacity: nargs,
        args: args_ptr,
    });
    let contref = new_cont_ref(Box::into_raw(contobj));
    contref as *mut u8 // TODO(dhil): we need memory clean up of
                       // continuation reference objects.
}

/// TODO
#[inline(always)]
pub fn resume(instance: &mut Instance, contraw: *mut u8) -> Result<u32, TrapReason> {
    let contref = contraw as *mut ContinuationReference;
    match unsafe { (*contref).0 } {
        None => Err(TrapReason::user_with_backtrace(anyhow::Error::msg(
            "Continuation is already taken",
        ))),
        Some(contobj) => {
            unsafe { *contref = ContinuationReference(None) };
            let fiber = unsafe { (*contobj).fiber };
            let fiber_stack = unsafe { &fiber.as_ref().unwrap().stack() };
            let tsp = TopOfStackPointer::as_raw(instance.tsp());
            unsafe { fiber_stack.write_parent(tsp) };
            instance.set_tsp(TopOfStackPointer::from_raw(fiber_stack.top().unwrap()));
            unsafe {
                (*(*(*instance.store()).vmruntime_limits())
                    .stack_limit
                    .get_mut()) = 0
            };
            match unsafe { fiber.as_mut().unwrap().resume(()) } {
                Ok(()) => {
                    // The result of the continuation was written to the first
                    // entry of the payload store by virtue of using the array
                    // calling trampoline to execute it.

                    Ok(0) // zero value = return normally.
                }
                Err(tag) => {
                    // We set the high bit to signal a return via suspend. We
                    // encode the tag into the remainder of the integer.
                    let signal_mask = 0xf000_0000;
                    debug_assert_eq!(tag & signal_mask, 0);
                    unsafe {
                        let cont_store_ptr = instance.get_typed_continuations_store_mut()
                            as *mut *mut ContinuationObject;
                        cont_store_ptr.write(contobj)
                    };
                    Ok(tag | signal_mask)
                }
            }
        }
    }
}

/// TODO
#[inline(always)]
pub fn suspend(instance: &mut Instance, tag_index: u32) {
    let stack_ptr = TopOfStackPointer::as_raw(instance.tsp());
    let parent = unsafe { stack_ptr.cast::<*mut u8>().offset(-2).read() };
    instance.set_tsp(TopOfStackPointer::from_raw(parent));
    let suspend = wasmtime_fibre::unix::Suspend::from_top_ptr(stack_ptr);
    suspend.switch::<(), u32, ()>(wasmtime_fibre::RunResult::Yield(tag_index))
}
