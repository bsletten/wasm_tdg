use std::error::Error;
use wasmtime::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Config::new();
    config.wasm_reference_types(true);
    
    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, ());
    let module = Module::from_file(&engine, "extref.wat")?;

    let instance = Instance::new(&mut store, &module, &[])?;

    let eref = ExternRef::new("secret key");
    let arr : [u8; 4] = [1, 2, 3, 4];
    
    let eref2 = ExternRef::new(arr);


    let table = instance.get_table(&mut store, "table").unwrap();
    table.set(&mut store, 3, Some(eref.clone()).into())?;
    table.set(&mut store, 4, Some(eref2.clone()).into())?;

    let ret = table.get(&mut store, 3)
        .unwrap()
        .unwrap_externref()
        .unwrap();

    let ret2 = table.get(&mut store, 4)
        .unwrap()
        .unwrap_externref()
        .unwrap();

    let str = *ret.data().downcast_ref::<&'static str>().unwrap();
    let arr2 = *ret2.data().downcast_ref::<[u8; 4]>().unwrap();

    println!("Retrieved external reference: {} from table slot {}", str, 3);
    println!("Retrieved external reference: {:?} from table slot {}", arr2, 4);

    let func = instance.get_typed_func::<Option<ExternRef>, Option<ExternRef>, _>
        (&mut store, "func")?;

    let ret = func.call(&mut store, Some(eref.clone()))?;

    let str2 = *(ret.unwrap()).data().downcast_ref::<&'static str>().unwrap();

    println!("Received {} back from calling extern-ref aware function.", str2);

    Ok(())
}
