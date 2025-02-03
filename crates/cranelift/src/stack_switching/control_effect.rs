use cranelift_codegen::ir;
use cranelift_codegen::ir::types::{I32, I64};
use cranelift_codegen::ir::InstBuilder;
use cranelift_frontend::FunctionBuilder;
use wasmtime_environ::stack_switching as stack_switching_environ;

/// Universal control effect. This structure encodes return signal,
/// resume signal, suspension signal, and suspension tags into a
/// pointer. This instance is used at compile time. There is a runtime
/// counterpart in `continuations/src/lib.rs`.
#[derive(Clone, Copy)]
pub struct ControlEffect(ir::Value);

impl ControlEffect {
    // Returns the discriminant
    pub fn signal<'a>(
        &self,
        _env: &mut crate::func_environ::FuncEnvironment<'a>,
        builder: &mut FunctionBuilder,
    ) -> ir::Value {
        builder.ins().ushr_imm(self.0, 32)
    }

    pub fn from_u64(val: ir::Value) -> Self {
        Self(val)
    }

    pub fn to_u64(&self) -> ir::Value {
        self.0
    }

    pub fn encode_resume<'a>(
        _env: &mut crate::func_environ::FuncEnvironment<'a>,
        builder: &mut FunctionBuilder,
    ) -> Self {
        let discriminant = builder.ins().iconst(
            I64,
            stack_switching_environ::CONTROL_EFFECT_RESUME_DISCRIMINANT as i64,
        );
        let val = builder.ins().ishl_imm(discriminant, 32);

        Self(val)
    }

    pub fn encode_switch<'a>(
        _env: &mut crate::func_environ::FuncEnvironment<'a>,
        builder: &mut FunctionBuilder,
    ) -> Self {
        let discriminant = builder.ins().iconst(
            I64,
            stack_switching_environ::CONTROL_EFFECT_SWITCH_DISCRIMINANT as i64,
        );
        let val = builder.ins().ishl_imm(discriminant, 32);

        Self(val)
    }

    pub fn encode_suspend<'a>(
        _env: &mut crate::func_environ::FuncEnvironment<'a>,
        builder: &mut FunctionBuilder,
        handler_index: ir::Value,
    ) -> Self {
        let discriminant = builder.ins().iconst(
            I64,
            stack_switching_environ::CONTROL_EFFECT_SUSPEND_DISCRIMINANT as i64,
        );
        let val = builder.ins().ishl_imm(discriminant, 32);
        let handler_index = builder.ins().uextend(I64, handler_index);
        let val = builder.ins().bor(val, handler_index);

        Self(val)
    }

    // Returns the payload of the `Suspend` variant
    pub fn handler_index<'a>(
        self,
        _env: &mut crate::func_environ::FuncEnvironment<'a>,
        builder: &mut FunctionBuilder,
    ) -> ir::Value {
        builder.ins().ireduce(I32, self.0)
    }
}
