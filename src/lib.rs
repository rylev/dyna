use anyhow::Context;
use component::dyna::dynamic_component::{Host, HostEngine};

wasmtime::component::bindgen!({
    with: {
        "component:dyna/dynamic-component/engine": Engine,
        "component:dyna/dynamic-component/component": ComponentState,
        "component:dyna/dynamic-component/type-item": TypeItem,
    }
});

pub use wasmtime::Engine;

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
            .table()
            .get(&self_)
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
            .table()
            .get_mut(&self_)
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(state.call_method(&name, args))
    }

    fn reflect(
        &mut self,
        self_: wasmtime::component::Resource<component::dyna::dynamic_component::Component>,
    ) -> wasmtime::Result<Vec<component::dyna::dynamic_component::ExportItem>> {
        let state = self
            .table()
            .get(&self_)
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(state.resolver.clone().reflect(self.table()))
    }

    fn drop(
        &mut self,
        self_: wasmtime::component::Resource<component::dyna::dynamic_component::Component>,
    ) -> wasmtime::Result<()> {
        self.table().delete(self_).unwrap();
        Ok(())
    }
}

impl<T> component::dyna::dynamic_component::HostTypeItem for T
where
    T: DynamicComponentView,
{
    fn kind(
        &mut self,
        self_: wasmtime::component::Resource<TypeItem>,
    ) -> wasmtime::Result<component::dyna::dynamic_component::TypeItemKind> {
        let typ = self
            .table()
            .get(&self_)
            .map_err(|e| anyhow::anyhow!("{e}"))?
            .clone();
        Ok(typ.convert_type(self.table()))
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<component::dyna::dynamic_component::TypeItem>,
    ) -> wasmtime::Result<()> {
        self.table().delete(rep)?;
        Ok(())
    }
}

pub struct ComponentState {
    store: wasmtime::Store<()>,
    instance: wasmtime::component::Instance,
    resolver: Resolver,
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
            resolver: Resolver { resolve, world_id },
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
}

#[derive(Clone)]
struct Resolver {
    resolve: wit_parser::Resolve,
    world_id: wit_parser::WorldId,
}

impl Resolver {
    fn reflect(
        &self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> Vec<component::dyna::dynamic_component::ExportItem> {
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
                    component::dyna::dynamic_component::ExportItem {
                        name: name.clone(),
                        kind: component::dyna::dynamic_component::ExportKind::Interface(
                            component::dyna::dynamic_component::Interface {
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
                            },
                        ),
                    }
                }
                wit_parser::WorldItem::Function(f) => {
                    component::dyna::dynamic_component::ExportItem {
                        name: key.clone().unwrap_name(),
                        kind: component::dyna::dynamic_component::ExportKind::Function(
                            convert_function(f, &self.resolve, resource_table),
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

fn convert_function(
    function: &wit_parser::Function,
    resolve: &wit_parser::Resolve,
    resource_table: &mut wasmtime::component::ResourceTable,
) -> component::dyna::dynamic_component::Function {
    component::dyna::dynamic_component::Function {
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
pub struct TypeItem {
    resolve: wit_parser::Resolve,
    typ: wit_parser::Type,
}

impl TypeItem {
    fn convert_type(
        &self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> component::dyna::dynamic_component::TypeItemKind {
        use component::dyna::dynamic_component::{
            EnumType, RecordType, ResultType, TypeItemKind, VariantType,
        };
        use wit_parser::Type::*;
        match &self.typ {
            String => TypeItemKind::String,
            Bool => TypeItemKind::Bool,
            U8 => TypeItemKind::U8,
            U16 => TypeItemKind::U16,
            U32 => TypeItemKind::U32,
            U64 => TypeItemKind::U64,
            S8 => TypeItemKind::S8,
            S16 => TypeItemKind::S16,
            S32 => TypeItemKind::S32,
            S64 => TypeItemKind::S64,
            Float32 => TypeItemKind::F32,
            Float64 => TypeItemKind::F64,
            Char => TypeItemKind::Char,
            Id(i) => {
                let typ = &self.resolve.types.get(*i).unwrap();
                match &typ.kind {
                    wit_parser::TypeDefKind::Tuple(t) => TypeItemKind::Tuple(
                        t.types
                            .iter()
                            .map(|t| push_type(t, &self.resolve, resource_table).unwrap())
                            .collect(),
                    ),
                    wit_parser::TypeDefKind::Option(o) => {
                        TypeItemKind::Option(push_type(o, &self.resolve, resource_table).unwrap())
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
                        TypeItemKind::Result(ResultType { ok, err })
                    }
                    wit_parser::TypeDefKind::List(l) => {
                        TypeItemKind::List(push_type(l, &self.resolve, resource_table).unwrap())
                    }
                    wit_parser::TypeDefKind::Enum(_) => TypeItemKind::Enum(EnumType {
                        name: typ.name.clone().unwrap(),
                    }),
                    wit_parser::TypeDefKind::Variant(_) => TypeItemKind::Variant(VariantType {
                        name: typ.name.clone().unwrap(),
                    }),
                    wit_parser::TypeDefKind::Record(_) => TypeItemKind::Record(RecordType {
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
) -> Result<
    wasmtime::component::Resource<component::dyna::dynamic_component::TypeItem>,
    wasmtime::component::ResourceTableError,
> {
    let typ = TypeItem {
        resolve: resolve.clone(),
        typ: typ.clone(),
    };
    resource_table.push(typ)
}
