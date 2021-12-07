(module
    (func $how_old (param $year_now i32) (param $year_born i32) (result i32)
        get_local $year_now
        get_local $year_born
        i32.sub)

    (export "how_old" (func $how_old))
)