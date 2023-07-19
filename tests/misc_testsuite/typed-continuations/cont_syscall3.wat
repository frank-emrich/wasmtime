(module
    (import "wasi_snapshot_preview1" "clock_time_get"
        (func $__wasi_clock_time_get (param i32 i64 i32) (result i32)))
    (func $run (export "run")

            ;;(call $__wasi_clock_time_get (i32.const 1) (i64.const 0) (i32.const 0))
            ;;(drop)




    )
    (memory (export "memory") 1)
    ;;
    (type $f (func))
    (type $ct (cont $f))

    (func $test  (export "test")
          (resume $ct (cont.new $ct (ref.func $run)))
    )
)
