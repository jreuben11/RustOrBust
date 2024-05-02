(module
  ;; import object + properties passed in from JS
  (import "console" "log" (func $log))
  (import "console" "error" (func $error))

  ;; write to wasm memory 
  (memory $mem 1)
  (data (i32.const 0) "Hi")

  ;; import memory from js
  (memory (import "js" "mem") 1)

  ;; function callable from js
  (func $sum (param $a i32) (param $b i32) (result i32)
    call $log
    call $error
    local.get $a
    local.get $b
    i32.add
  )

  ;; export memories and functions
  (export "mem" (memory $mem))
  (export "sum" (func $sum))
)