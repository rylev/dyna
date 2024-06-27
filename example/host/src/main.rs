#[tokio::main]
async fn main() {
    let dynamic_guest_path = std::env::args()
        .nth(1)
        .expect("expected path to component binary");

    let engine = create_engine().unwrap();

    let mut store = wasmtime::Store::new(&engine, State::new());
    let dynamic_component =
        wasmtime::component::Component::from_file(&engine, dynamic_guest_path).unwrap();
    let mut linker = wasmtime::component::Linker::new(&engine);
    dyna::add_to_linker(&mut linker).unwrap();
    wasmtime_wasi::add_to_linker_async(&mut linker).unwrap();
    let instance = linker
        .instantiate_async(&mut store, &dynamic_component)
        .await
        .unwrap();
    let func = instance
        .get_typed_func::<(), ()>(&mut store, "hello")
        .unwrap();
    func.call_async(&mut store, ()).await.unwrap();
}

fn create_engine() -> wasmtime::Result<wasmtime::Engine> {
    let mut config = wasmtime::Config::new();
    config.async_support(true).wasm_component_model(true);
    wasmtime::Engine::new(&config)
}

struct State {
    table: wasmtime::component::ResourceTable,
    ctx: wasmtime_wasi::WasiCtx,
}

impl State {
    fn new() -> Self {
        let dir_perms = wasmtime_wasi::DirPerms::all();
        let file_perms = wasmtime_wasi::FilePerms::all();
        Self {
            table: wasmtime::component::ResourceTable::new(),
            ctx: wasmtime_wasi::WasiCtxBuilder::new()
                .preopened_dir(".", "/", dir_perms, file_perms)
                .unwrap()
                .inherit_stdio()
                .build(),
        }
    }
}

impl wasmtime_wasi::WasiView for State {
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut wasmtime_wasi::WasiCtx {
        &mut self.ctx
    }
}

impl dyna::DynamicComponentView for State {
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable {
        &mut self.table
    }
}
