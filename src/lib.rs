use std::sync::Arc;

use anyhow::Context;
use component::dyna::{dynamic_component, wit};

wasmtime::component::bindgen!({
    with: {
        "component:dyna/dynamic-component/engine": Engine,
        "component:dyna/dynamic-component/component": ComponentState,
        "component:dyna/wit/world": Resolver,
        "component:dyna/wit/type": Type,
    }
});

pub use wasmtime::Engine;

/// Add the dynamic component to the linker.
pub fn add_to_linker<T: DynamicComponentView>(
    linker: &mut wasmtime::component::Linker<T>,
) -> anyhow::Result<()> {
    dynamic_component::add_to_linker(linker, |x| x)?;
    wit::add_to_linker(linker, |x| x)
}

/// A trait for hosting dynamic components.
pub trait DynamicComponentView {
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable;
}

impl<T> dynamic_component::HostEngine for T
where
    T: DynamicComponentView,
{
    fn new(
        &mut self,
    ) -> wasmtime::Result<wasmtime::component::Resource<dynamic_component::Engine>> {
        let mut config = wasmtime::Config::new();
        config.wasm_component_model(true);
        let engine = wasmtime::Engine::new(&config)?;
        self.table()
            .push(engine)
            .context("failed to allocate engine resource")
    }

    fn load_component(
        &mut self,
        self_: wasmtime::component::Resource<dynamic_component::Engine>,
        bytes: Vec<u8>,
    ) -> wasmtime::Result<
        Result<
            wasmtime::component::Resource<dynamic_component::Component>,
            dynamic_component::LoadError,
        >,
    > {
        let engine = self
            .table()
            .get(&self_)
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        let component_state = match ComponentState::load(engine, bytes) {
            Ok(c) => c,
            Err(e) => return Ok(Err(e)),
        };

        let resource = self
            .table()
            .push(component_state)
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(Ok(resource))
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<dynamic_component::Engine>,
    ) -> wasmtime::Result<()> {
        let _ = self.table().delete(rep)?;
        Ok(())
    }
}

impl<T> dynamic_component::Host for T where T: DynamicComponentView {}

impl<T> dynamic_component::HostComponent for T
where
    T: DynamicComponentView,
{
    fn call(
        &mut self,
        self_: wasmtime::component::Resource<dynamic_component::Component>,
        name: String,
        args: Vec<dynamic_component::Val>,
    ) -> wasmtime::Result<Result<Vec<dynamic_component::Val>, dynamic_component::CallError>> {
        let state = self
            .table()
            .get_mut(&self_)
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(state.call_method(&name, args))
    }

    fn world(
        &mut self,
        self_: wasmtime::component::Resource<dynamic_component::Component>,
    ) -> wasmtime::Result<
        Result<wasmtime::component::Resource<wit::World>, dynamic_component::ResolveError>,
    > {
        let state = self
            .table()
            .get(&self_)
            .map_err(|e| anyhow::anyhow!("{e}"))?;

        let Ok(wit_component::DecodedWasm::Component(resolve, world_id)) =
            wit_component::decode(&state.bytes)
        else {
            return Ok(Err(dynamic_component::ResolveError::InvalidBytes(
                "found wit package instead of the expect WebAssembly component".into(),
            )));
        };
        let resolve = Resolver {
            resolve: Arc::new(resolve),
            world_id,
        };
        let world = self
            .table()
            .push(resolve)
            .context("failed to allocate resolver resource")?;
        Ok(Ok(world))
    }

    fn drop(
        &mut self,
        self_: wasmtime::component::Resource<dynamic_component::Component>,
    ) -> wasmtime::Result<()> {
        self.table().delete(self_).unwrap();
        Ok(())
    }
}

impl<T> wit::Host for T where T: DynamicComponentView {}
impl<T> wit::HostWorld for T
where
    T: DynamicComponentView,
{
    fn exports(
        &mut self,
        self_: wasmtime::component::Resource<wit::World>,
    ) -> wasmtime::Result<Vec<wit::Export>> {
        let resolve = self
            .table()
            .get(&self_)
            .context("failed to get resolver resource")?;
        Ok(resolve.clone().exports(self.table()))
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<wit::World>) -> wasmtime::Result<()> {
        let _ = self.table().delete(rep)?;
        Ok(())
    }
}

impl<T> wit::HostType for T
where
    T: DynamicComponentView,
{
    fn kind(
        &mut self,
        self_: wasmtime::component::Resource<wit::Type>,
    ) -> wasmtime::Result<wit::TypeKind> {
        let typ = self
            .table()
            .get(&self_)
            .map_err(|e| anyhow::anyhow!("{e}"))?
            .clone();
        Ok(typ.convert_type(self.table()))
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<wit::Type>) -> wasmtime::Result<()> {
        self.table().delete(rep)?;
        Ok(())
    }
}

pub struct ComponentState {
    store: wasmtime::Store<()>,
    instance: wasmtime::component::Instance,
    bytes: Vec<u8>,
}

impl ComponentState {
    fn load(
        engine: &wasmtime::Engine,
        bytes: Vec<u8>,
    ) -> Result<Self, dynamic_component::LoadError> {
        let mut store = wasmtime::Store::new(engine, ());
        let component = wasmtime::component::Component::new(engine, &bytes)
            .map_err(|e| dynamic_component::LoadError::InvalidBytes(e.to_string()))?;
        let linker = wasmtime::component::Linker::new(engine);
        let instance = linker.instantiate(&mut store, &component).unwrap();
        Ok(ComponentState {
            instance,
            store,
            bytes,
        })
    }

    fn call_method(
        &mut self,
        name: &str,
        args: Vec<dynamic_component::Val>,
    ) -> Result<Vec<dynamic_component::Val>, dynamic_component::CallError> {
        let func = self
            .instance
            .get_func(&mut self.store, &name)
            .ok_or(dynamic_component::CallError::NoFunction)?;
        let params = args
            .into_iter()
            .map(|a| match a {
                dynamic_component::Val::String(s) => wasmtime::component::Val::String(s.into()),
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
                wasmtime::component::Val::String(s) => dynamic_component::Val::String(s.into()),
                _ => todo!(""),
            })
            .collect())
    }
}

#[derive(Clone)]
pub struct Resolver {
    resolve: Arc<wit_parser::Resolve>,
    world_id: wit_parser::WorldId,
}

impl Resolver {
    fn exports(&self, resource_table: &mut wasmtime::component::ResourceTable) -> Vec<wit::Export> {
        let world = self.world();
        world
            .exports
            .iter()
            .map(|(key, item)| match item {
                wit_parser::WorldItem::Interface(i) => {
                    let interface = self.resolve.interfaces.get(*i).unwrap();
                    let name = match key {
                        wit_parser::WorldKey::Name(name) => name,
                        wit_parser::WorldKey::Interface(_) => interface
                            .name
                            .as_ref()
                            .expect("non-inlined interface still did not have a name"),
                    };
                    wit::Export {
                        name: name.clone(),
                        kind: wit::ExportKind::Interface(wit::Interface {
                            functions: interface
                                .functions
                                .iter()
                                .map(|(name, func)| {
                                    (
                                        name.clone(),
                                        convert_function(func, &self.resolve, resource_table),
                                    )
                                })
                                .collect(),
                        }),
                    }
                }
                wit_parser::WorldItem::Function(f) => wit::Export {
                    name: key.clone().unwrap_name(),
                    kind: wit::ExportKind::Function(convert_function(
                        f,
                        &self.resolve,
                        resource_table,
                    )),
                },
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

fn convert_function(
    function: &wit_parser::Function,
    resolve: &wit_parser::Resolve,
    resource_table: &mut wasmtime::component::ResourceTable,
) -> wit::Function {
    wit::Function {
        params: function
            .params
            .iter()
            .map(|(param_name, param_type)| {
                (
                    param_name.clone(),
                    push_type(param_type, resolve, resource_table).unwrap(),
                )
            })
            .collect(),
        result: match &function.results {
            wit_parser::Results::Named(_) => todo!("Handle named results"),
            wit_parser::Results::Anon(t) => push_type(t, resolve, resource_table).unwrap(),
        },
    }
}

#[derive(Clone)]
pub struct Type {
    resolve: wit_parser::Resolve,
    typ: wit_parser::Type,
}

impl Type {
    fn convert_type(
        &self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> wit::TypeKind {
        use component::dyna::wit::{EnumType, RecordType, ResultType, TypeKind, VariantType};
        use wit_parser::Type::*;
        match &self.typ {
            String => TypeKind::String,
            Bool => TypeKind::Bool,
            U8 => TypeKind::U8,
            U16 => TypeKind::U16,
            U32 => TypeKind::U32,
            U64 => TypeKind::U64,
            S8 => TypeKind::S8,
            S16 => TypeKind::S16,
            S32 => TypeKind::S32,
            S64 => TypeKind::S64,
            Float32 => TypeKind::F32,
            Float64 => TypeKind::F64,
            Char => TypeKind::Char,
            Id(i) => {
                let typ = &self.resolve.types.get(*i).unwrap();
                match &typ.kind {
                    wit_parser::TypeDefKind::Tuple(t) => TypeKind::Tuple(
                        t.types
                            .iter()
                            .map(|t| push_type(t, &self.resolve, resource_table).unwrap())
                            .collect(),
                    ),
                    wit_parser::TypeDefKind::Option(o) => {
                        TypeKind::Option(push_type(o, &self.resolve, resource_table).unwrap())
                    }
                    wit_parser::TypeDefKind::Result(r) => {
                        let ok =
                            r.ok.as_ref()
                                .map(|t| push_type(t, &self.resolve, resource_table))
                                .transpose()
                                .unwrap();
                        let err =
                            r.ok.as_ref()
                                .map(|t| push_type(t, &self.resolve, resource_table))
                                .transpose()
                                .unwrap();
                        TypeKind::Result(ResultType { ok, err })
                    }
                    wit_parser::TypeDefKind::List(l) => {
                        TypeKind::List(push_type(l, &self.resolve, resource_table).unwrap())
                    }
                    wit_parser::TypeDefKind::Enum(_) => TypeKind::Enum(EnumType {
                        name: typ.name.clone().unwrap(),
                    }),
                    wit_parser::TypeDefKind::Variant(_) => TypeKind::Variant(VariantType {
                        name: typ.name.clone().unwrap(),
                    }),
                    wit_parser::TypeDefKind::Record(_) => TypeKind::Record(RecordType {
                        name: typ.name.clone().unwrap(),
                    }),
                    wit_parser::TypeDefKind::Resource => todo!("handle resources"),
                    wit_parser::TypeDefKind::Type(t) => Self {
                        resolve: self.resolve.clone(),
                        typ: t.clone(),
                    }
                    .convert_type(resource_table),
                    wit_parser::TypeDefKind::Unknown => {
                        panic!("Unresolved package found in resolver")
                    }
                    t => {
                        panic!("'{t:?}' type is not supported")
                    }
                }
            }
        }
    }
}

fn push_type(
    typ: &wit_parser::Type,
    resolve: &wit_parser::Resolve,
    resource_table: &mut wasmtime::component::ResourceTable,
) -> Result<wasmtime::component::Resource<wit::Type>, wasmtime::component::ResourceTableError> {
    let typ = Type {
        resolve: resolve.clone(),
        typ: typ.clone(),
    };
    resource_table.push(typ)
}
