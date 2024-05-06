use tracing::error;
use wasmer::{imports, Instance, Module, Store, Value};

fn main() -> anyhow::Result<()> {
    // let module_wat = r#"
    // (module
    //     (type $t0 (func (param i32) (result i32)))
    //     (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
    //         get_local $p0
    //         i32.const 1
    //         i32.add))
    // "#;

    common_x::log::init_log_filter("info");

    let mut store = Store::default();
    let module = Module::from_file(&store, "plugin/pkg/plugin_bg.wasm").map_err(|e| {
        error!("0: {e}");
        e
    })?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object).map_err(|e| {
        error!("1: {e}");
        e
    })?;

    let add_one = instance.exports.get_function("add").map_err(|e| {
        error!("2: {e}");
        e
    })?;
    let result = add_one
        .call(&mut store, &[Value::I32(1), Value::I32(1)])
        .map_err(|e| {
            error!("3: {e}");
            e
        })?;
    assert_eq!(result[0], Value::I32(2));

    Ok(())
}
