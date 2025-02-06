use wasmtime_asm_macros::asm_func;

asm_func!(
    "wasmtime_continuation_start",
    "

        ldr x3, [sp], #16
        ldr x2, [sp], #16
        ldr x1, [sp], #16
        ldr x0, [sp], #16

        // Call fiber_start
        bl {fiber_start}

        // Return to parent continuation
        // X29 (FP) is callee-saved and still contains TOS - 0x10
        // Use it to restore parent frame state

        ldr x1, [x29, #8]
        ldr x2, [x29, #-8]
        mov sp, x2
        ldr x29, [x29]


        mov x0, #0

        // Branch to return address
        br x1
    ",
    fiber_start = sym super::fiber_start,
);
