use std::error::Error;
use wasmtime::*;

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::from_file(&engine, "mvr.wat")?;

    let callback_func = Func::wrap(&mut store, |a: i32, b: i32| -> (i32, i32) {
        (b, a)
    });
    
    let instance = Instance::new(&mut store, &module, &[callback_func.into()])?;

    let myfunc = instance.get_typed_func::<(i32,i32), (i32, i32), _>(&mut store, "myfunc")?;
    let (a, b) = myfunc.call(&mut store, (13, 43))?;

    println!("Swapping {} {} produces {} {}", 13, 43, a, b);

    Ok(())
}
