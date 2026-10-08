//! The *provider* half of the composition: it implements `wasmtdg:calculator/ages`
//! and imports nothing at all. It has no `main`, no WASI, and no idea who will
//! call it.

wit_bindgen::generate!({
    world: "calculator",
    path: "wit",
});

struct Component;

impl exports::wasmtdg::calculator::ages::Guest for Component {
    fn how_old(year_now: i32, year_born: i32) -> i32 {
        if year_born <= year_now {
            year_now - year_born
        } else {
            -1
        }
    }
}

export!(Component);
