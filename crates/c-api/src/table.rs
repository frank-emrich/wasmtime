use crate::{
    handle_result, wasm_extern_t, wasm_ref_t, wasm_store_t, wasm_tabletype_t, wasmtime_error_t,
    wasmtime_val_t, CStoreContext, CStoreContextMut,
};
use anyhow::anyhow;
use std::mem::MaybeUninit;
use wasmtime::{Extern, HeapType, Ref, Table, TableType};

#[derive(Clone)]
#[repr(transparent)]
pub struct wasm_table_t {
    ext: wasm_extern_t,
}

wasmtime_c_api_macros::declare_ref!(wasm_table_t);

pub type wasm_table_size_t = u32;

impl wasm_table_t {
    pub(crate) fn try_from(e: &wasm_extern_t) -> Option<&wasm_table_t> {
        match &e.which {
            Extern::Table(_) => Some(unsafe { &*(e as *const _ as *const _) }),
            _ => None,
        }
    }

    fn table(&self) -> Table {
        match self.ext.which {
            Extern::Table(t) => t,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

fn option_wasm_ref_t_to_ref(r: Option<&wasm_ref_t>, table_ty: &TableType) -> Ref {
    match (r.map(|r| r.r.clone()), table_ty.element().heap_type()) {
        (None, HeapType::NoFunc | HeapType::Func | HeapType::Concrete(_)) => Ref::Func(None),
        (None, HeapType::Extern) => Ref::Extern(None),
        (Some(r), _) => r,
        (_, HeapType::NoCont | HeapType::Cont) => unimplemented!(), // TODO(dhil): We need embedder API support in wasmtime first.
    }
}

#[no_mangle]
pub unsafe extern "C" fn wasm_table_new(
    store: &mut wasm_store_t,
    tt: &wasm_tabletype_t,
    init: Option<&wasm_ref_t>,
) -> Option<Box<wasm_table_t>> {
    let tt = tt.ty().ty.clone();
    let init = option_wasm_ref_t_to_ref(init, &tt);
    let table = Table::new(store.store.context_mut(), tt, init).ok()?;
    Some(Box::new(wasm_table_t {
        ext: wasm_extern_t {
            store: store.store.clone(),
            which: table.into(),
        },
    }))
}

#[no_mangle]
pub unsafe extern "C" fn wasm_table_type(t: &wasm_table_t) -> Box<wasm_tabletype_t> {
    let table = t.table();
    let store = t.ext.store.context();
    Box::new(wasm_tabletype_t::new(table.ty(&store)))
}

#[no_mangle]
pub unsafe extern "C" fn wasm_table_get(
    t: &mut wasm_table_t,
    index: wasm_table_size_t,
) -> Option<Box<wasm_ref_t>> {
    let table = t.table();
    let r = table.get(t.ext.store.context_mut(), index)?;
    wasm_ref_t::new(r)
}

#[no_mangle]
pub unsafe extern "C" fn wasm_table_set(
    t: &mut wasm_table_t,
    index: wasm_table_size_t,
    r: Option<&wasm_ref_t>,
) -> bool {
    let table = t.table();
    let val = option_wasm_ref_t_to_ref(r, &table.ty(t.ext.store.context()));
    table.set(t.ext.store.context_mut(), index, val).is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn wasm_table_size(t: &wasm_table_t) -> wasm_table_size_t {
    let table = t.table();
    let store = t.ext.store.context();
    table.size(&store)
}

#[no_mangle]
pub unsafe extern "C" fn wasm_table_grow(
    t: &mut wasm_table_t,
    delta: wasm_table_size_t,
    init: Option<&wasm_ref_t>,
) -> bool {
    let table = t.table();
    let init = option_wasm_ref_t_to_ref(init, &table.ty(t.ext.store.context()));
    table.grow(t.ext.store.context_mut(), delta, init).is_ok()
}

#[no_mangle]
pub extern "C" fn wasm_table_as_extern(t: &mut wasm_table_t) -> &mut wasm_extern_t {
    &mut t.ext
}

#[no_mangle]
pub extern "C" fn wasm_table_as_extern_const(t: &wasm_table_t) -> &wasm_extern_t {
    &t.ext
}

#[no_mangle]
pub unsafe extern "C" fn wasmtime_table_new(
    store: CStoreContextMut<'_>,
    tt: &wasm_tabletype_t,
    init: &wasmtime_val_t,
    out: &mut Table,
) -> Option<Box<wasmtime_error_t>> {
    handle_result(
        init.to_val()
            .ref_()
            .ok_or_else(|| anyhow!("wasmtime_table_new init value is not a reference"))
            .and_then(|init| Table::new(store, tt.ty().ty.clone(), init)),
        |table| *out = table,
    )
}

#[no_mangle]
pub unsafe extern "C" fn wasmtime_table_type(
    store: CStoreContext<'_>,
    table: &Table,
) -> Box<wasm_tabletype_t> {
    Box::new(wasm_tabletype_t::new(table.ty(store)))
}

#[no_mangle]
pub extern "C" fn wasmtime_table_get(
    store: CStoreContextMut<'_>,
    table: &Table,
    index: u32,
    ret: &mut MaybeUninit<wasmtime_val_t>,
) -> bool {
    match table.get(store, index) {
        Some(r) => {
            crate::initialize(ret, wasmtime_val_t::from_val(r.into()));
            true
        }
        None => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn wasmtime_table_set(
    store: CStoreContextMut<'_>,
    table: &Table,
    index: u32,
    val: &wasmtime_val_t,
) -> Option<Box<wasmtime_error_t>> {
    handle_result(
        val.to_val()
            .ref_()
            .ok_or_else(|| anyhow!("wasmtime_table_set value is not a reference"))
            .and_then(|val| table.set(store, index, val)),
        |()| {},
    )
}

#[no_mangle]
pub extern "C" fn wasmtime_table_size(store: CStoreContext<'_>, table: &Table) -> u32 {
    table.size(store)
}

#[no_mangle]
pub unsafe extern "C" fn wasmtime_table_grow(
    store: CStoreContextMut<'_>,
    table: &Table,
    delta: u32,
    val: &wasmtime_val_t,
    prev_size: &mut u32,
) -> Option<Box<wasmtime_error_t>> {
    handle_result(
        val.to_val()
            .ref_()
            .ok_or_else(|| anyhow!("wasmtime_table_grow value is not a reference"))
            .and_then(|val| table.grow(store, delta, val)),
        |prev| *prev_size = prev,
    )
}
