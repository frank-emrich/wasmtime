file target/debug/wasmtime
set args -Ddebug-info -Daddress-map -W=exceptions,function-references,typed-continuations luna.wasm

define go
  source debug.gdb
  run
end

source debug.py

del
break run.rs:842
command $bpnum
  hunt $bpnum
end
