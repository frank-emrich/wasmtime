//! Helper functions and structures for the translation.
use crate::environ::TargetEnvironment;
use crate::WasmResult;
use core::u32;
use cranelift_codegen::ir;
use cranelift_frontend::FunctionBuilder;
use wasmparser::{FuncValidator, WasmModuleResources};
use wasmtime_types::WasmValType;

/// Get the parameter and result types for the given Wasm blocktype.
pub fn blocktype_params_results<'a, T>(
    validator: &'a FuncValidator<T>,
    ty: wasmparser::BlockType,
) -> WasmResult<(
    impl ExactSizeIterator<Item = wasmparser::ValType> + Clone + 'a,
    impl ExactSizeIterator<Item = wasmparser::ValType> + Clone + 'a,
)>
where
    T: WasmModuleResources,
{
    return Ok(match ty {
        wasmparser::BlockType::Empty => (
            itertools::Either::Left(std::iter::empty()),
            itertools::Either::Left(None.into_iter()),
        ),
        wasmparser::BlockType::Type(ty) => (
            itertools::Either::Left(std::iter::empty()),
            itertools::Either::Left(Some(ty).into_iter()),
        ),
        wasmparser::BlockType::FuncType(ty_index) => {
            let ty = validator
                .resources()
                .sub_type_at(ty_index)
                .expect("should be valid")
                .unwrap_func();

            (
                itertools::Either::Right(ty.params().iter().copied()),
                itertools::Either::Right(ty.results().iter().copied()),
            )
        }
    });
}

/// Create a `Block` with the given Wasm parameters.
pub fn block_with_params<PE: TargetEnvironment + ?Sized>(
    builder: &mut FunctionBuilder,
    params: impl IntoIterator<Item = wasmparser::ValType>,
    environ: &PE,
) -> WasmResult<ir::Block> {
    let block = builder.create_block();
    for ty in params {
        match ty {
            wasmparser::ValType::I32 => {
                builder.append_block_param(block, ir::types::I32);
            }
            wasmparser::ValType::I64 => {
                builder.append_block_param(block, ir::types::I64);
            }
            wasmparser::ValType::F32 => {
                builder.append_block_param(block, ir::types::F32);
            }
            wasmparser::ValType::F64 => {
                builder.append_block_param(block, ir::types::F64);
            }
            wasmparser::ValType::Ref(rt) => {
                let hty = environ.convert_heap_type(rt.heap_type());
                let (ty, needs_stack_map) = environ.reference_type(hty);
                let val = builder.append_block_param(block, ty);
                if needs_stack_map {
                    builder.declare_value_needs_stack_map(val);
                }
            }
            wasmparser::ValType::V128 => {
                builder.append_block_param(block, ir::types::I8X16);
            }
        }
    }
    Ok(block)
}

/// Turns a `wasmparser` `f32` into a `Cranelift` one.
pub fn f32_translation(x: wasmparser::Ieee32) -> ir::immediates::Ieee32 {
    ir::immediates::Ieee32::with_bits(x.bits())
}

/// Turns a `wasmparser` `f64` into a `Cranelift` one.
pub fn f64_translation(x: wasmparser::Ieee64) -> ir::immediates::Ieee64 {
    ir::immediates::Ieee64::with_bits(x.bits())
}

/// Special VMContext value label. It is tracked as 0xffff_fffe label.
pub fn get_vmctx_value_label() -> ir::ValueLabel {
    const VMCTX_LABEL: u32 = 0xffff_fffe;
    ir::ValueLabel::from_u32(VMCTX_LABEL)
}

/// Create a `Block` with the given Wasm parameters.
pub fn block_with_params_wasmtype<PE: TargetEnvironment + ?Sized>(
    builder: &mut FunctionBuilder,
    params: &[WasmValType],
    environ: &PE,
) -> WasmResult<ir::Block> {
    let block = builder.create_block();
    for ty in params {
        match ty {
            WasmValType::I32 => {
                builder.append_block_param(block, ir::types::I32);
            }
            WasmValType::I64 => {
                builder.append_block_param(block, ir::types::I64);
            }
            WasmValType::F32 => {
                builder.append_block_param(block, ir::types::F32);
            }
            WasmValType::F64 => {
                builder.append_block_param(block, ir::types::F64);
            }
            WasmValType::Ref(rt) => {
                let (rt, _) = environ.reference_type(rt.heap_type);
                builder.append_block_param(block, rt);
            }
            WasmValType::V128 => {
                builder.append_block_param(block, ir::types::I8X16);
            }
        }
    }
    Ok(block)
}

/// Compute the maximum number of arguments that a suspension may
/// supply.
pub fn resumetable_max_num_tag_payloads<PE: crate::FuncEnvironment + ?Sized>(
    tags: &[u32],
    environ: &PE,
) -> WasmResult<usize> {
    Ok(tags
        .iter()
        .map(|tag| environ.tag_params(*tag).len())
        .max()
        .unwrap())
}
