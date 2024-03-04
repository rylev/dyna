use anyhow::Context;
use component::dyna::dynamic_component::Host;

wasmtime::component::bindgen!();

/// Add the dynamic component to the linker.
pub fn add_to_linker<T: DynamicComponentView>(
    linker: &mut wasmtime::component::Linker<T>,
) -> anyhow::Result<()> {
    component::dyna::dynamic_component::add_to_linker(linker, |x| x)
}

/// A trait for hosting dynamic components.
pub trait DynamicComponentView {
    fn engine(&self) -> &wasmtime::Engine;
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable;
}

impl<T> Host for T
where
    T: DynamicComponentView,
{
    fn load_component(
        &mut self,
        path: String,
    ) -> wasmtime::Result<
        wasmtime::component::Resource<component::dyna::dynamic_component::Component>,
    > {
        println!("Loading component from path: {}", path);
        let mut store = wasmtime::Store::new(self.engine(), ());
        let component = wasmtime::component::Component::from_file(self.engine(), path).unwrap();
        let linker = wasmtime::component::Linker::new(self.engine());
        let instance = linker.instantiate(&mut store, &component).unwrap();
        let component_state = ComponentState { instance, store };
        let resource = self
            .table()
            .push(component_state)
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(wasmtime::component::Resource::new_own(resource.rep()))
    }
}

impl<T> component::dyna::dynamic_component::HostComponent for T
where
    T: DynamicComponentView,
{
    fn call(
        &mut self,
        self_: wasmtime::component::Resource<component::dyna::dynamic_component::Component>,
        name: String,
        args: Vec<component::dyna::dynamic_component::Val>,
    ) -> wasmtime::Result<Vec<component::dyna::dynamic_component::Val>> {
        let self_ = wasmtime::component::Resource::new_borrow(self_.rep());
        let state: &mut ComponentState = self.table().get_mut(&self_).unwrap();
        let func = state
            .instance
            .get_func(&mut state.store, &name)
            .with_context(|| format!("failed to find function export `{name}`"))?;
        let params = args
            .into_iter()
            .map(|a| match a {
                component::dyna::dynamic_component::Val::Str(s) => {
                    wasmtime::component::Val::String(s.into())
                }
            })
            .collect::<Vec<_>>();
        let mut result = [wasmtime::component::Val::String(
            String::from("").into_boxed_str(),
        )];
        func.call(&mut state.store, &params, &mut result)?;
        let results = result
            .into_iter()
            .map(|v| match v {
                wasmtime::component::Val::String(s) => {
                    component::dyna::dynamic_component::Val::Str(s.into())
                }
                _ => todo!(""),
            })
            .collect();
        Ok(results)
    }

    fn drop(
        &mut self,
        self_: wasmtime::component::Resource<component::dyna::dynamic_component::Component>,
    ) -> wasmtime::Result<()> {
        let self_: wasmtime::component::Resource<ComponentState> =
            wasmtime::component::Resource::new_own(self_.rep());
        self.table().delete(self_).unwrap();
        Ok(())
    }
}

struct ComponentState {
    store: wasmtime::Store<()>,
    instance: wasmtime::component::Instance,
}
