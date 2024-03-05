use anyhow::Context;
use component::dyna::dynamic_component::{Host, HostEngine};
use wit_parser::Type;

wasmtime::component::bindgen!();

/// Add the dynamic component to the linker.
pub fn add_to_linker<T: DynamicComponentView>(
    linker: &mut wasmtime::component::Linker<T>,
) -> anyhow::Result<()> {
    component::dyna::dynamic_component::add_to_linker(linker, |x| x)
}

/// A trait for hosting dynamic components.
pub trait DynamicComponentView {
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable;
}

impl<T> HostEngine for T
where
    T: DynamicComponentView,
{
    fn new(
        &mut self,
    ) -> wasmtime::Result<wasmtime::component::Resource<component::dyna::dynamic_component::Engine>>
    {
        let mut config = wasmtime::Config::new();
        config.wasm_component_model(true);
        let engine = wasmtime::Engine::new(&config)?;
        let resource = self
            .table()
            .push(engine)
            .context("failed to allocate engine resource")?;
        Ok(wasmtime::component::Resource::new_own(resource.rep()))
    }

    fn load_component(
        &mut self,
        self_: wasmtime::component::Resource<component::dyna::dynamic_component::Engine>,
        bytes: Vec<u8>,
    ) -> wasmtime::Result<
        Result<
            wasmtime::component::Resource<component::dyna::dynamic_component::Component>,
            component::dyna::dynamic_component::LoadError,
        >,
    > {
        let engine = self
            .borrow_engine(self_)
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        let component_state = match ComponentState::load(engine, &bytes) {
            Ok(c) => c,
            Err(e) => return Ok(Err(e)),
        };

        let resource = self
            .table()
            .push(component_state)
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(Ok(wasmtime::component::Resource::new_own(resource.rep())))
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<component::dyna::dynamic_component::Engine>,
    ) -> wasmtime::Result<()> {
        let _ = self
            .table()
            .delete::<wasmtime::Engine>(wasmtime::component::Resource::new_own(rep.rep()))?;
        Ok(())
    }
}

impl<T> Host for T where T: DynamicComponentView {}

impl<T> component::dyna::dynamic_component::HostComponent for T
where
    T: DynamicComponentView,
{
    fn call(
        &mut self,
        self_: wasmtime::component::Resource<component::dyna::dynamic_component::Component>,
        name: String,
        args: Vec<component::dyna::dynamic_component::Val>,
    ) -> wasmtime::Result<
        Result<
            Vec<component::dyna::dynamic_component::Val>,
            component::dyna::dynamic_component::CallError,
        >,
    > {
        let state = self
            .borrow_component(self_)
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(state.call_method(&name, args))
    }

    fn reflect(
        &mut self,
        self_: wasmtime::component::Resource<component::dyna::dynamic_component::Component>,
    ) -> Result<Vec<component::dyna::dynamic_component::ExportItem>, anyhow::Error> {
        let state = self
            .borrow_component(self_)
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(state.reflect())
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
    resolve: wit_parser::Resolve,
    world_id: wit_parser::WorldId,
}

impl ComponentState {
    fn load(
        engine: &wasmtime::Engine,
        bytes: &[u8],
    ) -> Result<Self, component::dyna::dynamic_component::LoadError> {
        let mut store = wasmtime::Store::new(engine, ());
        let component = wasmtime::component::Component::new(engine, bytes).map_err(|e| {
            component::dyna::dynamic_component::LoadError::InvalidBytes(e.to_string())
        })?;
        let linker = wasmtime::component::Linker::new(engine);
        let instance = linker.instantiate(&mut store, &component).unwrap();

        let Ok(wit_component::DecodedWasm::Component(resolve, world_id)) =
            wit_component::decode(bytes)
        else {
            return Err(component::dyna::dynamic_component::LoadError::InvalidBytes(
                "found wit package instead of the expect WebAssembly component".into(),
            ));
        };
        Ok(ComponentState {
            instance,
            store,
            resolve,
            world_id,
        })
    }

    fn call_method(
        &mut self,
        name: &str,
        args: Vec<component::dyna::dynamic_component::Val>,
    ) -> Result<
        Vec<component::dyna::dynamic_component::Val>,
        component::dyna::dynamic_component::CallError,
    > {
        let func = self
            .instance
            .get_func(&mut self.store, &name)
            .ok_or(component::dyna::dynamic_component::CallError::NoFunction)?;
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
        func.call(&mut self.store, &params, &mut result)
            .expect("TODO: other error");
        Ok(result
            .into_iter()
            .map(|v| match v {
                wasmtime::component::Val::String(s) => {
                    component::dyna::dynamic_component::Val::Str(s.into())
                }
                _ => todo!(""),
            })
            .collect())
    }

    fn reflect(&self) -> Vec<component::dyna::dynamic_component::ExportItem> {
        self.world()
            .exports
            .iter()
            .map(|(key, item)| match item {
                wit_parser::WorldItem::Interface(_) => todo!(),
                wit_parser::WorldItem::Function(f) => {
                    component::dyna::dynamic_component::ExportItem {
                        name: key.clone().unwrap_name(),
                        kind: component::dyna::dynamic_component::ExportKind::Function(
                            component::dyna::dynamic_component::Function {
                                params: f
                                    .params
                                    .iter()
                                    .map(|(param_name, param_type)| {
                                        let param_type = match param_type {
                                            Type::String => {
                                                component::dyna::dynamic_component::TypeItem::Str
                                            }
                                            _ => todo!(),
                                        };
                                        (param_name.clone(), param_type)
                                    })
                                    .collect(),
                                results: component::dyna::dynamic_component::TypeItem::Str,
                            },
                        ),
                    }
                }
                wit_parser::WorldItem::Type(_) => todo!(),
            })
            .collect()
    }

    fn world(&self) -> &wit_parser::World {
        self.resolve
            .worlds
            .get(self.world_id)
            .expect("world_id is not found in the resolved wit package")
    }
}

trait EngineExtension {
    fn borrow_engine(
        &mut self,
        resource: wasmtime::component::Resource<component::dyna::dynamic_component::Engine>,
    ) -> Result<&wasmtime::Engine, wasmtime::component::ResourceTableError>;
}

impl<T> EngineExtension for T
where
    T: DynamicComponentView,
{
    fn borrow_engine(
        &mut self,
        resource: wasmtime::component::Resource<component::dyna::dynamic_component::Engine>,
    ) -> Result<&wasmtime::Engine, wasmtime::component::ResourceTableError> {
        let self_ = wasmtime::component::Resource::new_borrow(resource.rep());
        self.table().get(&self_)
    }
}

trait ComponentExtension {
    fn borrow_component(
        &mut self,
        resource: wasmtime::component::Resource<component::dyna::dynamic_component::Component>,
    ) -> Result<&mut ComponentState, wasmtime::component::ResourceTableError>;
}

impl<T> ComponentExtension for T
where
    T: DynamicComponentView,
{
    fn borrow_component(
        &mut self,
        resource: wasmtime::component::Resource<component::dyna::dynamic_component::Component>,
    ) -> Result<&mut ComponentState, wasmtime::component::ResourceTableError> {
        let self_ = wasmtime::component::Resource::new_borrow(resource.rep());
        self.table().get_mut(&self_)
    }
}
