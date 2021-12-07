(module
  (func $add (param $a i32) (param $b i32) (result i32)
      get_local $a
      get_local $b
      i32.add)

  (func $sub (param $a i32) (param $b i32) (result i32)
      get_local $a
      get_local $b
      i32.sub)

  (table (export "tbl") anyfunc (elem $add $sub))
)