#[tokio::main]
async fn main() {
    let dynamic_guest_path = std::env::args()
        .skip(1)
        .next()
        .expect("expected path to component binary");

    let engine = create_engine().unwrap();

    let mut store = wasmtime::Store::new(&engine, State::new());
    let component = wasmtime::component::Component::from_file(&engine, dynamic_guest_path).unwrap();
    let mut linker = wasmtime::component::Linker::new(&engine);
    dyna::add_to_linker(&mut linker).unwrap();
    wasmtime_wasi::preview2::command::add_to_linker(&mut linker).unwrap();
    let instance = linker
        .instantiate_async(&mut store, &component)
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
    ctx: wasmtime_wasi::preview2::WasiCtx,
}

impl State {
    fn new() -> Self {
        let dir = cap_std::fs::Dir::open_ambient_dir(".", cap_std::ambient_authority()).unwrap();
        let dir_perms = wasmtime_wasi::preview2::DirPerms::all();
        let file_perms = wasmtime_wasi::preview2::FilePerms::all();
        Self {
            table: wasmtime::component::ResourceTable::new(),
            ctx: wasmtime_wasi::preview2::WasiCtxBuilder::new()
                .preopened_dir(dir, dir_perms, file_perms, "/")
                .inherit_stdio()
                .build(),
        }
    }
}

impl wasmtime_wasi::preview2::WasiView for State {
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut wasmtime_wasi::preview2::WasiCtx {
        &mut self.ctx
    }
}

impl dyna::DynamicComponentView for State {
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable {
        &mut self.table
    }
}
