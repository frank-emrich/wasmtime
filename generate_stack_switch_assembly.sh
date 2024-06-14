#!/bin/bash

#

cat > test.s <<EOF
# fn(
#    save_stack_ptr_addr(rdi): *mut u8
#                              Where to save the fake frame ptr
#    target_stack_ptr(rsi) : *mut u8
#                            Where to load the frame pointer to jump to
#    in_payload(r8) : u64
# )
.intel_syntax noprefix
.section .text
.globl _start
_start:

# This assumes that the "resume address" is already in RAX
push rax

# We're switching to arbitrary code somewhere else, so pessimistically
# assume that all callee-save register are clobbered. This means we need
# to save/restore all of them.
#
# Note that this order for saving is important since we use CFI directives
# below to point to where all the saved registers are.
#
# The frame pointer must come first, so that we have the return address
# and then the frame pointer on the stack (at addresses called 0xCFF8
# and 0xCFF0, respectively, in the second picture in unix.rs)
push rbp
mov rbp, rsp
push rbx # at -0x08[rbp]
push r12 # at -0x10[rbp]
push r13 # at -0x18[rbp]
push r14 # at -0x20[rbp]
push r15 # at -0x28[rbp]

# Load the resume frame pointer that we're going to resume at and
# store where we're going to get resumed from.
mov rax, [rsi]
mov [rdi], rbp


# Swap stacks: We loaded the resume frame pointer into RAX, meaning
# that it is near the beginning of the pseudo frame of the invocation of
# wasmtime_fibe_switch that we want to get back to.
# Thus, we need to turn this *frame* pointer back into the
# corresponding *stack* pointer. This is simple: The resume frame
# pointer is where wamtime_fibre_switch stored RBP, and we want to
# calculate the stack pointer after it pushed the next 5 registers, too.
#
# Using the values from the second picture in unix.rs: If we loaded
# 0xCFF0 into RAX, then we want to set RSP to 0xCFC8. Thus, to reflect
# that an additional 5 registers where pushed on the stack after RBP, we
# subtract 5 * 8 = 0x28 from RAX.
lea rsp, -0x28[rax]
# Restore callee-saved registers
pop r15
pop r14
pop r13
pop r12
pop rbx
pop rbp

# Backwards compatibility with wasmtime_fibre_start,
# which expects TOS in RDI
add rdi, 0x10

# We return the payload (i.e., the third argument to this function)
mov rax, r8

ret
EOF

as -o test.o test.s
ld -Ttext 200000 --oformat binary -o test.bin test.o
objdump --disassembler-options=intel -D -b binary -m i386:x86-64 test.bin | tee test.text
#      0:       55                      push   rbp

python3 - <<EOF
import re

INDENT = 12

def emit(bytes, instr):

    # print(bytes)
    slice = "[" + ", ".join(bytes) + "]"

    print(" " * INDENT + f"// {instr.strip()}:")
    print(" " * INDENT + f"sink.put_data(&{slice}); ")
    #print()

file = open('test.text', 'r')
lines  = file.readlines()
r = r'([ a-f0-9]{8}):\s*([ 0-9a-f]{20})(.*)'
print("Emission code:")
byte_count = 0
for l in lines:
    m = re.match(r, l)
    if m:
        offset, bytes, instr = m.groups()
        offset = int(offset, 16)
        if offset >= 200000:
            break
        bytes = bytes.strip().split(" ")
        bytes = list(map(lambda b: f"0x{b}", bytes))
        assert offset == byte_count
        byte_count += len(bytes)
        emit(bytes, instr)

print(f"Final number of bytes - 8: {hex(byte_count - 7)}")
EOF
#       0:       55                      push   rbp
#       0:       55                      push   rbp
#48 8d 05 00 00 00 00
