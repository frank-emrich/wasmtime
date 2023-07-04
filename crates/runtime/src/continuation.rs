//! Continuations TODO

use crate::instance::TopOfStackPointer;
use crate::vmcontext::{VMArrayCallFunction, VMFuncRef, VMOpaqueContext, ValRaw};
use crate::{Instance, TrapReason};
use std::cmp;
use std::mem;
use std::ptr;
use wasmtime_fibre::{Fiber, FiberStack, Suspend};

type ContinuationFiber = Fiber<'static, (), u32, ()>;
type Yield = Suspend<(), u32, ()>;

/// Similar to Vector<*mut u128>, but allowing us to hand out the `data` pointer to generated code.
struct Payload {
    length: usize,
    capacity: usize,
    /// This is null if and only if capacity (and thus also payload_length) are 0.
    data: *mut u128,
}

impl Payload {
    fn empty() -> Payload {
        return Payload {
            length: 0,
            capacity: 0,
            data: ptr::null_mut(),
        };
    }

    fn ensure_capacity(&mut self, requested_capacity: usize) {
        assert!(requested_capacity != 0);

        if self.capacity == 0 {
            assert!(unsafe { self.data.as_ref() }.is_none());
            assert!(self.length == 0);
            let mut vec = Vec::with_capacity(requested_capacity);
            let ptr = vec.as_mut_ptr();
            self.data = ptr;
        } else if requested_capacity > self.capacity {
            let mut vec = unsafe { Vec::from_raw_parts(self.data, self.length, self.capacity) };
            vec.resize(requested_capacity, 0);
            let ptr = vec.as_mut_ptr();
            self.data = ptr;
        }
    }
}

/// TODO
#[repr(C)]
pub struct ContinuationObject {
    fiber: *mut ContinuationFiber,

    /// Used to store payload data that is
    /// 1. supplied by cont.bind
    /// 2. supplied by resume
    /// 3. supplied when suspending to a tag
    payload: Payload,

    /// Becomes Some once the initial resume is executed.
    /// The enclosed pointer is null if and only if the function passed to `cont.new` has 0 parameters and return values.
    /// If the continuation finishes normally (i.e., the underlying function returns), this contains the returned data.
    results: Option<*mut u128>,
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
pub fn cont_obj_get_payloads(obj: *mut ContinuationObject) -> *mut u128 {
    // This panics if `data` is null, which is the case if `capacity` is 0.
    // This means that generated code acting on payloads must *not* work
    // in a way such that it unconditionally asks for the payload pointer
    // and then only accesses it if it actually needs to read or write payload data.
    assert!(unsafe { (*obj).payload.data.as_ref() }.is_some());
    unsafe { (*obj).payload.data }
}

/// TODO
#[inline(always)]
pub fn cont_obj_reset_payloads(obj: *mut ContinuationObject) {
    unsafe { (*obj).payload.length = 0 };
}

/// TODO
#[inline(always)]
pub fn cont_obj_get_results(obj: *mut ContinuationObject) -> *mut u128 {
    assert!(unsafe { (*obj).results.unwrap().as_ref().is_some() });
    unsafe { (*obj).results.unwrap() }
}

/// TODO
#[inline(always)]
pub fn cont_obj_get_next_free_payload_slot(obj: *mut ContinuationObject) -> *mut u128 {
    let args_len = unsafe { (*obj).payload.length };
    unsafe { (*obj).payload.data.offset(args_len as isize) }
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
    mem::drop(unsafe { (*contobj).fiber });
    unsafe {
        mem::drop((*contobj).payload.data);
        (*contobj).results.map_or((), mem::drop);
    }
    mem::drop(contobj)
}

/// TODO
#[inline(always)]
pub fn cont_obj_ensure_payloads_additional_capacity(obj: *mut ContinuationObject, capacity: usize) {
    let length = unsafe { (*obj).payload.length };
    unsafe { (*obj).payload.ensure_capacity(length + capacity) };
    // if unsafe { (*contobj).payloads.len() } < npayloads {
    //     Vec::resize(unsafe { &mut (*contobj).payloads }, npayloads, 0u128)
    // }
}

/// TODO
#[inline(always)]
pub fn cont_new(
    instance: &mut Instance,
    func: *mut u8,
    param_count: usize,
    result_count: usize,
) -> *mut ContinuationReference {
    let func = func as *mut VMFuncRef;
    let callee_ctx = unsafe { (*func).vmctx };
    let caller_ctx = VMOpaqueContext::from_vmcontext(instance.vmctx());
    let f = unsafe {
        mem::transmute::<
            VMArrayCallFunction,
            unsafe extern "C" fn(*mut VMOpaqueContext, *mut VMOpaqueContext, *mut ValRaw, usize),
        >((*func).array_call)
    };
    let capacity = cmp::max(param_count, result_count);

    let payload = if capacity == 0 {
        Payload::empty()
    } else {
        let mut args = Vec::with_capacity(capacity);
        Payload {
            length: 0,
            capacity,
            data: args.as_mut_ptr(),
        }
    };

    let args_ptr = payload.data;

    let contobj = Box::new(ContinuationObject {
        fiber: ptr::null_mut(),
        payload,
        results: None,
    });
    let contobj_ptr = Box::into_raw(contobj);

    let fiber = Box::new(
        Fiber::new(
            FiberStack::new(4096).unwrap(),
            move |_first_val: (), _suspend: &Yield| unsafe {
                let contobj = contobj_ptr.as_mut().unwrap();

                contobj.results = Some(contobj.payload.data);
                contobj.payload = Payload::empty();

                f(callee_ctx, caller_ctx, args_ptr as *mut ValRaw, param_count)
            },
        )
        .unwrap(),
    );

    unsafe {
        contobj_ptr.as_mut().unwrap().fiber = Box::into_raw(fiber);
    }

    new_cont_ref(contobj_ptr)
    // TODO(dhil): we need memory clean up of
    // continuation reference objects.
}

/// TODO
#[inline(always)]
pub fn resume(
    instance: &mut Instance,
    contref: *mut ContinuationReference,
) -> Result<u32, TrapReason> {
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
