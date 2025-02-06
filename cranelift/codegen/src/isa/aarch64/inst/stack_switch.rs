use crate::{isa::aarch64::inst::regs, machinst::Reg};

#[allow(dead_code)]
pub struct ControlContextLayout {
    pub size: usize,
    pub stack_pointer_offset: usize,
    pub frame_pointer_offset: usize,
    pub ip_offset: usize,
}

pub fn control_context_layout() -> ControlContextLayout {
    ControlContextLayout {
        size: 24,
        stack_pointer_offset: 0,
        frame_pointer_offset: 8,
        ip_offset: 16,
    }
}

pub fn payload_register() -> Reg {
    regs::xreg(0)
}
