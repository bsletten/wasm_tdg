(module
  (import "js" "table" (table 2 anyfunc))
  
  (type $sig (func (param $a i32) (param $b i32) (result i32)))
  
  (func (export "myadd") (param $a i32) (param $b i32) (result i32)
      (call_indirect (type $sig) (get_local $a) (get_local $b) (i32.const 0))
  )

  (func (export "mysub") (param $a i32) (param $b i32) (result i32)
      (call_indirect (type $sig) (get_local $a) (get_local $b) (i32.const 1))
  )
)