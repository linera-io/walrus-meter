(module
	(func $add_locals (param $x i32) (param $y i32) (result i32)
		(local $t i32)

		local.get $x
		local.get $y
		call $add
		local.set $t

		local.get $t
	)

	(func $add (param $x i32) (param $y i32) (result i32)
		(i32.add
			(local.get $x)
			(local.get $y)
		)
	)

	(start $start)
	(func $start
	    (call $add (i32.const 1) (i32.const 2))
		drop

	    (call $add_locals (i32.const 1) (i32.const 2))
		drop
	)
)
