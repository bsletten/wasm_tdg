(module
   (type $voidfunc (func))
   (import "hello" "world" (func $hello.world (type $voidfunc)))
   (func $exec
      call $hello.world
   )
   (export "exec" (func $exec))   
)
