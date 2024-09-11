// A WORD OF CAUTION
//
// This entire file basically needs to be kept in sync with itself. It's not
// really possible to modify just one bit of this file without understanding
// all the other bits. Documentation tries to reference various bits here and
// there but try to make sure to read over everything before tweaking things!

use wasmtime_asm_macros::asm_func;

// fn(
//    top_of_stack(rdi): *mut u8
//    payload(rsi) : u64
// )
//
// The payload (i.e., second argument) is return unchanged, allowing data to be
// passed from the continuation that calls `wasmtime_fibre_switch` to the one
// that subsequently runs.
//
// This function is only used to return to the parent stack.
asm_func!(
    "wasmtime_fibre_switch",
    "
        mov rbp, -0x10[rdi]
        mov rsp, -0x18[rdi]

        // We pass the payload (i.e., the second argument to this function) to
        // the parent stack. The stack_switch instruction uses RDI for this
        // purpose.
        xchg rdi, rsi

        jmp -0x08[rsi]
    ",
);

// fn(
//    top_of_stack(rdi): *mut u8,
//    func_ref(rsi): *const VMFuncRef,
//    caller_vmctx(rdx): *mut VMContext
//    args_ptr(rcx): *mut ValRaw
//    args_capacity(r8) : u64
// )
//
// This function installs the launchpad for the computation to run on the fiber,
// such that executing a `stack_switch` instruction on the stack actually runs the
// desired computation.
//
// Concretely, switching to the stack prepare by the `wasmtime_fibre_init`
// function evokes that we enter `wasmtime_fibre_start`, which then in turn
// calls `fiber_start` with a subset of the arguments above (namely: func_ref,
// caller_vmctx, args_ptr, args_capacity).
//
// The layout of the FiberStack near the top of stack (TOS) *after* running this
// function is as follows:
//
//  Offset from    |
//       TOS       | Contents
//  ---------------|-----------------------------------------------------------
//          -0x08   address of wasmtime_fibre_start function (future PC)
//          -0x10   TOS - 0x10 (future RBP)
//          -0x18   TOS - 0x40 (future RSP)
//          -0x20   0
//          -0x28   func_ref
//          -0x30   caller_vmctx
//          -0x38   args_ptr
//          -0x40   args_capacity
//          -0x48   undefined
#[rustfmt::skip]
asm_func!(
    "wasmtime_fibre_init",
    "
        // Here we're going to set up a control context  as expected by
        // `stack_switch` instruction. The values we store here will get restored into
        // registers by that function and the `wasmtime_fibre_start` function will
        // take over and understands which values are in which registers.
        //

        // Install wasmtime_fibre_start PC at TOS - 0x08
        lea r9, {start}[rip]
        mov -0x08[rdi], r9

        // Store TOS - 0x10 at TOS - 0x10
        // This is the frame pointer used in the bottommost frame within the stack
        lea rax, -0x10[rdi]
        mov -0x10[rdi], rax

        // Store TOS - 0x40 at TOS - 0x18
        // This is the stack pointer for the bottommost frame within the stack
        lea rax, -0x40[rdi]
        mov -0x18[rdi], rax

        // Alignment (and makes sure that wasmtime_fibre_start cannot return)
        mov qword ptr -0x20[rdi], 0

        // Install remaining arguments
        mov -0x28[rdi], rsi
        mov -0x30[rdi], rdx
        mov -0x38[rdi], rcx
        mov -0x40[rdi], r8

        ret
    ",
    start = sym super::wasmtime_fibre_start,
);

// This is a pretty special function that has no real signature. Its use is to
// be the "base" function of all fibers. This entrypoint is used in
// `wasmtime_fibre_init` to bootstrap the execution of a new fiber.
//
// We also use this function as a persistent frame on the stack to emit dwarf
// information to unwind into the caller. This allows us to unwind from the
// fiber's stack back to the main stack that the fiber was called from. We use
// special dwarf directives here to do so since this is a pretty nonstandard
// function.
//
// If you're curious a decent introduction to CFI things and unwinding is at
// https://www.imperialviolet.org/2017/01/18/cfi.html
//
// Note that this function is never called directly. It is only ever entered
// when a `stack_switch` instruction loads its address when switching to a stack
// prepared by `wasmtime_fibre_init`.
//
// Executing `stack_switch` on a stack prepared by `wasmtime_fibre_init`  as described in the
// comment on `wasmtime_fibre_init` leads to the following values in various
// registers when execution of wasmtime_fibre_start begins::
//
// RSP: TOS - 0x40
// RBP: TOS - 0x10
asm_func!(
    "wasmtime_fibre_start",
    "
        // Use the `simple` directive on the startproc here which indicates that
        // some default settings for the platform are omitted, since this
        // function is so nonstandard
        //.cfi_startproc simple
        //.cfi_def_cfa_offset 0

        // This is where things get special, we're specifying a custom dwarf
        // expression for how to calculate the CFA. The goal here is that we
        // need to load the parent's stack pointer just before the call it made
        // into `wasmtime_fibre_switch`. Note that the CFA value changes over
        // time as well because a fiber may be resumed multiple times from
        // different points on the original stack. This means that our custom
        // CFA directive involves `DW_OP_deref`, which loads data from memory.
        //
        // The expression we're encoding here is that the CFA, the stack pointer
        // of whatever performed `stack_switch`, is:
        //
        //        *$rsp + 0x10
        //
        // $rsp is the stack pointer of `wasmtime_fibre_start` at the time the
        // next instruction after the `.cfi_escape` is executed. Our $rsp at the
        // start of this function is 16 bytes below the top of the stack (0xAff0
        // in the diagram in unix.rs). The $rsp of our
        // parent invocation is stored at that location, so we dereference the
        // stack pointer to load it.
        //
        // After dereferencing, though, we have the $rsp value after performing
        // `stack_switch` instruction in the parent, thus we have the stored
        // RIP and RBP on the stack first.
        // Hence we offset another 0x10 bytes.
        // .cfi_escape 0x0f, /* DW_CFA_def_cfa_expression */ \
        //     4,            /* the byte length of this expression */ \
        //     0x57,         /* DW_OP_reg7 (rsp) */ \
        //     0x06,         /* DW_OP_deref */ \
        //     0x23, 0x10    /* DW_OP_plus_uconst 0x10 */

        // And now after we've indicated where our CFA is for our parent
        // function, we can define that where all of the saved registers are
        // located. This uses standard `.cfi` directives which indicate that
        // these registers are all stored relative to the CFA. Note that this
        // order is kept in sync with the above register spills in
        // `wasmtime_fibre_switch`.
        //.cfi_rel_offset rip, -8
        //.cfi_rel_offset rbp, -16


        // The body of this function is pretty similar. All our parameters are
        // already loaded into registers by the switch function. The
        // `wasmtime_fibre_init` routine arranged the various values to be
        // materialized into the registers used here. Our job is to then move
        // the values into the ABI-defined registers and call the entry-point
        // (i.e., the fiber_start function).
        // Note that `call` is used here to leave this frame on the stack so we
        // can use the dwarf info here for unwinding.
        //
        // Note that the next 5 instructions amount to calling fiber_start
        // with the following arguments:
        // 1. TOS
        // 2. func_ref
        // 3. caller_vmctx
        // 4. args_ptr
        // 5. args_capacity
        //
        // Note that fiber_start never returns: Instead, it // resume to the
        // parent FiberStack via wasmtime_fibre_switch.

        pop r8  // args_capacity
        pop rcx // args_ptr
        pop rdx // caller_vmctx
        pop rsi // func_ref
        lea rdi, 0x20[rsp] // TOS
        call {fiber_start}

        // We should never get here and purposely emit an invalid instruction.
        ud2
        //.cfi_endproc
    ",
    fiber_start = sym super::fiber_start,
);
