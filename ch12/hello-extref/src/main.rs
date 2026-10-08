use std::error::Error;
use wasmtime::*;

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::from_file(&engine, "extref.wat")?;

    let instance = Instance::new(&mut store, &module, &[])?;

    let eref = ExternRef::new(&mut store, "secret key")?;
    let arr : [u8; 4] = [1, 2, 3, 4];

    let eref2 = ExternRef::new(&mut store, arr)?;

    let table = instance.get_table(&mut store, "table").unwrap();
    table.set(&mut store, 3, Ref::Extern(Some(eref)))?;
    table.set(&mut store, 4, Ref::Extern(Some(eref2)))?;

    let ret = table.get(&mut store, 3)
        .unwrap()
        .unwrap_extern()
        .copied()
        .unwrap();

    let ret2 = table.get(&mut store, 4)
        .unwrap()
        .unwrap_extern()
        .copied()
        .unwrap();

    let str = *ret.data(&store)?.unwrap().downcast_ref::<&'static str>().unwrap();
    let arr2 = *ret2.data(&store)?.unwrap().downcast_ref::<[u8; 4]>().unwrap();

    println!("Retrieved external reference: {} from table slot {}", str, 3);
    println!("Retrieved external reference: {:?} from table slot {}", arr2, 4);

    let func = instance.get_typed_func::<Option<Rooted<ExternRef>>, Option<Rooted<ExternRef>>>
        (&mut store, "func")?;

    let ret = func.call(&mut store, Some(eref))?;

    let str2 = *ret.unwrap().data(&store)?.unwrap().downcast_ref::<&'static str>().unwrap();

    println!("Received {} back from calling extern-ref aware function.", str2);

    Ok(())
}
