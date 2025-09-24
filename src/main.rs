use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, fmt::time};
use wasmtime::*;
use wasmtime_wasi::{
    WasiCtxBuilder,
    p1::{self, WasiP1Ctx},
};

fn setup_tracing() {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .with_timer(time::UtcTime::rfc_3339())
        .init();
}

#[tokio::main]
async fn main() {
    setup_tracing();

    info!("wasm start");

    let engine = Engine::new(Config::default().async_support(true).debug_info(true)).unwrap();
    let module = Module::from_file(&engine, "./alpine_3.22.1.wasm").unwrap();

    let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
    p1::add_to_linker_async(&mut linker, |t| t).unwrap();
    let pre = linker.instantiate_pre(&module).unwrap();

    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_env()
        .arg("ls")
        .build_p1();

    let mut store = Store::new(&engine, wasi_ctx);

    let instance = pre.instantiate_async(&mut store).await.unwrap();

    let func = instance
        .get_typed_func::<(), ()>(&mut store, "_start")
        .unwrap();

    info!("will call_async");

    func.call_async(&mut store, ()).await.unwrap();

    info!("wasm done");
}
