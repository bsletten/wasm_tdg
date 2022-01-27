(module
  (import "js" "table" (table 2 funcref))
  
  (type $sig (func (param $a i32) (param $b i32) (result i32)))
  
  (func (export "myadd") (param $a i32) (param $b i32) (result i32)
      (call_indirect (type $sig) (local.get $a) (local.get $b) (i32.const 0))
  )

  (func (export "mysub") (param $a i32) (param $b i32) (result i32)
      (call_indirect (type $sig) (local.get $a) (local.get $b) (i32.const 1))
  )
)