(module
 (memory (import "js" "mem") 1)
 (func (export "fibonacci") (param $n i32)
    (local $index i32)
    (local $ptr i32)

    (i32.store (i32.const 0) (i32.const 0))
    (i32.store (i32.const 4) (i32.const 1))    
    
    (local.set $index (i32.const 2))
    (local.set $ptr (i32.const 8))

    (block $break
      (loop $top
            (br_if $break (i32.eq (local.get $n) (local.get $index)))
	    (i32.store
	       (local.get $ptr)
	       (i32.add
	         (i32.load (i32.sub (local.get $ptr) (i32.const 4)))
		 (i32.load (i32.sub (local.get $ptr) (i32.const 8)))
	       )
	    )
	    
	    (local.set $ptr (i32.add (local.get $ptr) (i32.const 4)))
	    (local.set $index (i32.add (local.get $index) (i32.const 1)))	    
	    (br $top)
      )
    )
   )
 )
