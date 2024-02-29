use anyhow::Context;
use component::dyna::dynamic_component::Host;

wasmtime::component::bindgen!(in "guest/wit");

#[tokio::main]
async fn main() {
    let component_path = std::env::args()
        .skip(1)
        .next()
        .expect("expected path to component binary");

    let engine = create_engine(true).unwrap();

    let mut store = wasmtime::Store::new(&engine, State::new(create_engine(false).unwrap()));
    let component = wasmtime::component::Component::from_file(&engine, component_path).unwrap();
    let mut linker = wasmtime::component::Linker::new(&engine);
    component::dyna::dynamic_component::add_to_linker(&mut linker, |x| x).unwrap();
    let instance = linker
        .instantiate_async(&mut store, &component)
        .await
        .unwrap();
    let func = instance
        .get_typed_func::<(), ()>(&mut store, "hello")
        .unwrap();
    func.call_async(&mut store, ()).await.unwrap();
}

struct State {
    engine: wasmtime::Engine,
    table: wasmtime::component::ResourceTable,
}

impl State {
    fn new(engine: wasmtime::Engine) -> Self {
        Self {
            engine,
            table: wasmtime::component::ResourceTable::new(),
        }
    }
}

impl Host for State {
    fn load_component(
        &mut self,
        path: String,
    ) -> wasmtime::Result<
        wasmtime::component::Resource<component::dyna::dynamic_component::Component>,
    > {
        let mut store = wasmtime::Store::new(&self.engine, ());
        let component = wasmtime::component::Component::from_file(&self.engine, path).unwrap();
        let linker = wasmtime::component::Linker::new(&self.engine);
        let instance = linker.instantiate(&mut store, &component).unwrap();
        let component_state = ComponentState { instance, store };
        let resource = self
            .table
            .push(component_state)
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(wasmtime::component::Resource::new_own(resource.rep()))
    }
}

fn create_engine(async_enabled: bool) -> wasmtime::Result<wasmtime::Engine> {
    let mut config = wasmtime::Config::new();
    if async_enabled {
        config.async_support(true);
    }
    config.wasm_component_model(true);
    wasmtime::Engine::new(&config)
}

impl component::dyna::dynamic_component::HostComponent for State {
    fn call(
        &mut self,
        self_: wasmtime::component::Resource<component::dyna::dynamic_component::Component>,
        name: String,
    ) -> wasmtime::Result<()> {
        let self_ = wasmtime::component::Resource::new_own(self_.rep());
        let state: &mut ComponentState = self.table.get_mut(&self_).unwrap();
        let func = state
            .instance
            .get_func(&mut state.store, &name)
            .with_context(|| format!("failed to find function export `{name}`"))?;
        let mut result = [wasmtime::component::Val::String(
            String::from("").into_boxed_str(),
        )];
        func.call(&mut state.store, &[], &mut result)?;
        println!("{result:?}");
        Ok(())
    }

    fn drop(
        &mut self,
        self_: wasmtime::component::Resource<component::dyna::dynamic_component::Component>,
    ) -> wasmtime::Result<()> {
        let self_: wasmtime::component::Resource<ComponentState> =
            wasmtime::component::Resource::new_own(self_.rep());
        self.table.delete(self_).unwrap();
        Ok(())
    }
}

struct ComponentState {
    store: wasmtime::Store<()>,
    instance: wasmtime::component::Instance,
}
