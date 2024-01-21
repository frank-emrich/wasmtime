//! `wasmtime-wasi` now supports using multiple snapshots to interface to the
//! same `WasiCtx`!
//!
//! `wasmtime_wasi::Wasi::new(&Store, WasiCtx)` is a struct which owns your
//! `WasiCtx` and provides linkage to every available snapshot.
//!
//! Individual snapshots are available through
//! `wasmtime_wasi::snapshots::preview_{0, 1}::Wasi::new(&Store, Rc<RefCell<WasiCtx>>)`.

#![warn(clippy::cast_sign_loss)]

#[cfg(feature = "preview2")]
pub mod preview2;

pub use wasi_common::{Error, I32Exit, WasiCtx, WasiDir, WasiFile};

/// Re-export the commonly used wasi-cap-std-sync crate here. This saves
/// consumers of this library from having to keep additional dependencies
/// in sync.
#[cfg(feature = "sync")]
pub mod sync {
    pub use wasi_cap_std_sync::*;
    use wasmtime::Linker;
    pub fn add_to_linker<T, U>(
        linker: &mut Linker<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()>
    where
        U: Send + wasi_common::snapshots::preview_0::wasi_unstable::WasiUnstable
            + wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1,
    {
        snapshots::preview_1::add_wasi_snapshot_preview1_to_linker(linker, get_cx)?;
        snapshots::preview_0::add_wasi_unstable_to_linker(linker, get_cx)?;
        Ok(())
    }
    pub mod snapshots {
        pub mod preview_1 {
            /// Adds all instance items to the specified `Linker`.
            pub fn add_wasi_snapshot_preview1_to_linker<T, U>(
                linker: &mut wiggle::wasmtime_crate::Linker<T>,
                get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wiggle::anyhow::Result<()>
            where
                U: wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1,
            {
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "args_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::args_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "args_sizes_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::args_sizes_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "environ_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::environ_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "environ_sizes_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::environ_sizes_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "clock_res_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::clock_res_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "clock_time_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i64,
                            arg2: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::clock_time_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_advise",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i64,
                            arg2: i64,
                            arg3: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_advise(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_allocate",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i64,
                            arg2: i64,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_allocate(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_close",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_close(
                                                ctx,
                                                &mem,
                                                arg0,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_datasync",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_datasync(
                                                ctx,
                                                &mem,
                                                arg0,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_fdstat_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_fdstat_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_fdstat_set_flags",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_fdstat_set_flags(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_fdstat_set_rights",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i64,
                            arg2: i64,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_fdstat_set_rights(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_filestat_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_filestat_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_filestat_set_size",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i64,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_filestat_set_size(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_filestat_set_times",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i64,
                            arg2: i64,
                            arg3: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_filestat_set_times(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_pread",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i64,
                            arg4: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_pread(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_prestat_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_prestat_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_prestat_dir_name",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_prestat_dir_name(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_pwrite",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i64,
                            arg4: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_pwrite(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_read",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_read(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_readdir",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i64,
                            arg4: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_readdir(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_renumber",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_renumber(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_seek",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i64,
                            arg2: i32,
                            arg3: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_seek(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_sync",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_sync(
                                                ctx,
                                                &mem,
                                                arg0,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_tell",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_tell(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "fd_write",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::fd_write(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "path_create_directory",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::path_create_directory(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "path_filestat_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::path_filestat_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "path_filestat_set_times",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i64,
                            arg5: i64,
                            arg6: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::path_filestat_set_times(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                                arg5,
                                                arg6,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "path_link",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                            arg5: i32,
                            arg6: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::path_link(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                                arg5,
                                                arg6,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "path_open",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                            arg5: i64,
                            arg6: i64,
                            arg7: i32,
                            arg8: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::path_open(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                                arg5,
                                                arg6,
                                                arg7,
                                                arg8,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "path_readlink",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                            arg5: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::path_readlink(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                                arg5,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "path_remove_directory",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::path_remove_directory(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "path_rename",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                            arg5: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::path_rename(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                                arg5,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "path_symlink",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::path_symlink(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "path_unlink_file",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::path_unlink_file(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "poll_oneoff",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::poll_oneoff(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "proc_exit",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                        | -> wiggle::anyhow::Result<()> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <()>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::proc_exit(
                                                ctx,
                                                &mem,
                                                arg0,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "proc_raise",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::proc_raise(
                                                ctx,
                                                &mem,
                                                arg0,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "sched_yield",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::sched_yield(
                                                ctx,
                                                &mem,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "random_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::random_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "sock_accept",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::sock_accept(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "sock_recv",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                            arg5: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::sock_recv(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                                arg5,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "sock_send",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::sock_send(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "sock_shutdown",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_1::wasi_snapshot_preview1::sock_shutdown(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                Ok(())
            }
        }
        pub mod preview_0 {
            /// Adds all instance items to the specified `Linker`.
            pub fn add_wasi_unstable_to_linker<T, U>(
                linker: &mut wiggle::wasmtime_crate::Linker<T>,
                get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wiggle::anyhow::Result<()>
            where
                U: wasi_common::snapshots::preview_0::wasi_unstable::WasiUnstable,
            {
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "args_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::args_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "args_sizes_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::args_sizes_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "environ_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::environ_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "environ_sizes_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::environ_sizes_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "clock_res_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::clock_res_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "clock_time_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i64,
                            arg2: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::clock_time_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_advise",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i64,
                            arg2: i64,
                            arg3: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_advise(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_allocate",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i64,
                            arg2: i64,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_allocate(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_close",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_close(
                                                ctx,
                                                &mem,
                                                arg0,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_datasync",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_datasync(
                                                ctx,
                                                &mem,
                                                arg0,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_fdstat_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_fdstat_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_fdstat_set_flags",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_fdstat_set_flags(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_fdstat_set_rights",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i64,
                            arg2: i64,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_fdstat_set_rights(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_filestat_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_filestat_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_filestat_set_size",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i64,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_filestat_set_size(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_filestat_set_times",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i64,
                            arg2: i64,
                            arg3: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_filestat_set_times(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_pread",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i64,
                            arg4: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_pread(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_prestat_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_prestat_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_prestat_dir_name",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_prestat_dir_name(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_pwrite",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i64,
                            arg4: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_pwrite(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_read",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_read(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_readdir",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i64,
                            arg4: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_readdir(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_renumber",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_renumber(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_seek",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i64,
                            arg2: i32,
                            arg3: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_seek(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_sync",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_sync(
                                                ctx,
                                                &mem,
                                                arg0,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_tell",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_tell(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "fd_write",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::fd_write(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "path_create_directory",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::path_create_directory(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "path_filestat_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::path_filestat_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "path_filestat_set_times",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i64,
                            arg5: i64,
                            arg6: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::path_filestat_set_times(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                                arg5,
                                                arg6,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "path_link",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                            arg5: i32,
                            arg6: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::path_link(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                                arg5,
                                                arg6,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "path_open",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                            arg5: i64,
                            arg6: i64,
                            arg7: i32,
                            arg8: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::path_open(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                                arg5,
                                                arg6,
                                                arg7,
                                                arg8,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "path_readlink",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                            arg5: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::path_readlink(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                                arg5,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "path_remove_directory",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::path_remove_directory(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "path_rename",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                            arg5: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::path_rename(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                                arg5,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "path_symlink",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::path_symlink(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "path_unlink_file",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::path_unlink_file(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "poll_oneoff",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::poll_oneoff(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "proc_exit",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                        | -> wiggle::anyhow::Result<()> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <()>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::proc_exit(
                                                ctx,
                                                &mem,
                                                arg0,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "proc_raise",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::proc_raise(
                                                ctx,
                                                &mem,
                                                arg0,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "sched_yield",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::sched_yield(
                                                ctx,
                                                &mem,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "random_get",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::random_get(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "sock_recv",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                            arg5: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::sock_recv(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                                arg5,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "sock_send",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::sock_send(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                                arg2,
                                                arg3,
                                                arg4,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                linker
                    .func_wrap(
                        "wasi_unstable",
                        "sock_shutdown",
                        move |
                            mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                            arg0: i32,
                            arg1: i32,
                        | -> wiggle::anyhow::Result<i32> {
                            let result = async {
                                let export = caller.get_export("memory");
                                let (mem, ctx) = match &export {
                                    Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                        let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                        let ctx = get_cx(ctx);
                                        (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                                    }
                                    Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                        let ctx = get_cx(caller.data_mut());
                                        (
                                            wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                            ctx,
                                        )
                                    }
                                    _ => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!("missing required memory export"),
                                            );
                                            error
                                        });
                                    }
                                };
                                Ok(
                                    <i32>::from(
                                        wasi_common::snapshots::preview_0::wasi_unstable::sock_shutdown(
                                                ctx,
                                                &mem,
                                                arg0,
                                                arg1,
                                            )
                                            .await?,
                                    ),
                                )
                            };
                            wiggle::run_in_dummy_executor(result)?
                        },
                    )?;
                Ok(())
            }
        }
    }
}

/// Sync mode is the "default" of this crate, so we also export it at the top
/// level.
#[cfg(feature = "sync")]
pub use sync::*;

/// Re-export the wasi-tokio crate here. This saves consumers of this library from having
/// to keep additional dependencies in sync.
#[cfg(feature = "tokio")]
pub mod tokio {
    pub use wasi_tokio::*;
    super::define_wasi!(async T: Send);
}

// The only difference between these definitions for sync vs async is whether
// the wasmtime::Funcs generated are async (& therefore need an async Store and an executor to run)
// or whether they have an internal "dummy executor" that expects the implementation of all
// the async funcs to poll to Ready immediately.
#[doc(hidden)]
#[macro_export]
macro_rules! define_wasi {
    ($async_mode:tt $($bounds:tt)*) => {

use wasmtime::Linker;

pub fn add_to_linker<T, U>(
    linker: &mut Linker<T>,
    get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
) -> anyhow::Result<()>
    where U: Send
            + wasi_common::snapshots::preview_0::wasi_unstable::WasiUnstable
            + wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1,
        $($bounds)*
{
    snapshots::preview_1::add_wasi_snapshot_preview1_to_linker(linker, get_cx)?;
    snapshots::preview_0::add_wasi_unstable_to_linker(linker, get_cx)?;
    Ok(())
}

pub mod snapshots {
    pub mod preview_1 {
        wiggle::wasmtime_integration!({
            // The wiggle code to integrate with lives here:
            target: wasi_common::snapshots::preview_1,
            // This must be the same witx document as used above. This should be ensured by
            // the `WASI_ROOT` env variable, which is set in wasi-common's `build.rs`.
            witx: ["$WASI_ROOT/phases/snapshot/witx/wasi_snapshot_preview1.witx"],
            errors: { errno => trappable Error },
            $async_mode: *
        });
    }
    pub mod preview_0 {
        wiggle::wasmtime_integration!({
            // The wiggle code to integrate with lives here:
            target: wasi_common::snapshots::preview_0,
            // This must be the same witx document as used above. This should be ensured by
            // the `WASI_ROOT` env variable, which is set in wasi-common's `build.rs`.
            witx: ["$WASI_ROOT/phases/old/snapshot_0/witx/wasi_unstable.witx"],
            errors: { errno => trappable Error },
            $async_mode: *
        });
    }
}
}
}

/// Exit the process with a conventional OS error code as long as Wasmtime
/// understands the error. If the error is not an `I32Exit` or `Trap`, return
/// the error back to the caller for it to decide what to do.
///
/// Note: this function is designed for usage where it is acceptable for
/// Wasmtime failures to terminate the parent process, such as in the Wasmtime
/// CLI; this would not be suitable for use in multi-tenant embeddings.
#[cfg(feature = "exit")]
pub fn maybe_exit_on_error(e: anyhow::Error) -> anyhow::Error {
    use std::process;
    use wasmtime::Trap;

    // If a specific WASI error code was requested then that's
    // forwarded through to the process here without printing any
    // extra error information.
    let code = e
        .downcast_ref::<I32Exit>()
        .map(|e| e.0)
        .or_else(|| e.downcast_ref::<preview2::I32Exit>().map(|e| e.0));
    if let Some(exit) = code {
        // Print the error message in the usual way.
        // On Windows, exit status 3 indicates an abort (see below),
        // so return 1 indicating a non-zero status to avoid ambiguity.
        if cfg!(windows) && exit >= 3 {
            process::exit(1);
        }
        process::exit(exit);
    }

    // If the program exited because of a trap, return an error code
    // to the outside environment indicating a more severe problem
    // than a simple failure.
    if e.is::<Trap>() {
        eprintln!("Error: {:?}", e);

        if cfg!(unix) {
            // On Unix, return the error code of an abort.
            process::exit(128 + libc::SIGABRT);
        } else if cfg!(windows) {
            // On Windows, return 3.
            // https://docs.microsoft.com/en-us/cpp/c-runtime-library/reference/abort?view=vs-2019
            process::exit(3);
        }
    }

    e
}
