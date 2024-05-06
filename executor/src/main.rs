use tracing::error;
use wasi_common::sync::WasiCtxBuilder;
use wasmtime::*;

fn main() -> Result<()> {
    common_x::log::init_log_filter("info");

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    let module =
        Module::from_file(&engine, "target/wasm32-wasi/release/hello.wasm").map_err(|e| {
            error!("failed to load module: {}", e);
            e
        })?;
    linker.module(&mut store, "", &module)?;

    let instantiate = linker.instantiate(&mut store, &module)?;
    instantiate
        .get_typed_func::<(), ()>(&mut store, "_start")?
        .call(&mut store, ())?;

    Ok(())
}
