use std::error::Error;
use wasmtime::*;

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::from_file(&engine, "hello.wat")?;
    let instance = Instance::new(&mut store, &module, &[])?;

    let how_old = instance.get_typed_func::<(i32,i32), (i32), _>(&mut store, "how_old")?;
    let age : i32 = how_old.call(&mut store, (2021i32, 2000i32))?;

    println!("You are {}", age);

    Ok(())
}
