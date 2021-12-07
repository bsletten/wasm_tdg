(module
  (import "js" "table" (table 2 anyfunc))

  (func $add (param $a i32) (param $b i32) (result i32)
      get_local $a
      get_local $b
      i32.add)

  (func $sub (param $a i32) (param $b i32) (result i32)
      get_local $a
      get_local $b
      i32.sub)

  (elem (i32.const 0) $add)
  (elem (i32.const 1) $sub)
)