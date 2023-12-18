set debuginfod enabled on
add-auto-load-safe-path /home/frank/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/etc
set substitute-path /rustc/79e9716c980570bfd1f666e3b16ac583f0168962 /home/frank/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust

set disassembly-flavor intel


#source debug.gdb
file target/debug/wasmtime
set args -Ddebug-info -Daddress-map -W=exceptions,function-references,typed-continuations luna.wasm
