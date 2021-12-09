(module
   (func $swap (import "" "swap") (param i32 i32) (result i32 i32))

   (func $myfunc (export "myfunc") (param i32 i32) (result i32 i32)
      (call $swap (local.get 0) (local.get 1))
   )
)