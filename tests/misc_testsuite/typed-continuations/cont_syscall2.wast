(module
  ;; (type (;0;) (func (param i32 i32 i32) (result i32)))
  ;; (type (;1;) (func (param i32 i64 i32) (result i64)))
  ;; (type (;2;) (func (param i32) (result i32)))
  ;; (type (;3;) (func (param i32)))
  ;; (type $three (func (param i32)))

  ;; (type (;4;) (func (param i32 i32) (result i32)))
  ;; (type (;5;) (func (param i32 i64 i32 i32) (result i32)))
  ;; (type (;6;) (func (param i32 i32 i32 i32) (result i32)))
  ;; (type $seven (func (param i32 i32 i32 i32) (result i32)))
  ;; (type (;7;) (func))
  ;; (type (;8;) (func (result i32)))
  ;; (import "wasi_snapshot_preview1" "args_get" (func (;0;) (type 4)))
  ;; (import "wasi_snapshot_preview1" "args_sizes_get" (func (;1;) (type 4)))
  ;; (import "wasi_snapshot_preview1" "fd_close" (func (;2;) (type 2)))
  ;; (import "wasi_snapshot_preview1" "fd_fdstat_get" (func (;3;) (type 4)))
  ;; (import "wasi_snapshot_preview1" "fd_seek" (func (;4;) (type 5)))
  ;; (import "wasi_snapshot_preview1" "fd_write" (func (;5;) (type 6)))
  (import "wasi_snapshot_preview1" "proc_exit"  (func $f (param i32)))


  (func $test (export "test") (result i32)
    ;;(resume $ct (cont.new $start))
    (call $f (i32.const 0))
    (i32.const 123)
  )



  ;;(export "_start" $test)

  )

(assert_return (invoke "test") (i32.const 100))
