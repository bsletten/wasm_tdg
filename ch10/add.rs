#[no_mangle]
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("2 + 3: {}", add(2,3));
}
