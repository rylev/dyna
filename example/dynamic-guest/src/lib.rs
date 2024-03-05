#[allow(dead_code)]
mod bindings;

use bindings::component::dyna::dynamic_component::Engine;
use bindings::Guest;

pub struct Component;

impl Guest for Component {
    fn hello() {
        let component_bytes = std::fs::read("example/static_guest.wasm").unwrap();
        let engine = Engine::new();
        let component = engine.load_component(&component_bytes).unwrap();
        let exports = component.reflect();
        println!("Exports:");
        for export in &exports {
            match &export.kind {
                bindings::component::dyna::dynamic_component::ExportKind::Function(f) => {
                    print_function(&export.name, f, 1)
                }
                bindings::component::dyna::dynamic_component::ExportKind::Interface(i) => {
                    println!("  interface {} {{", export.name);
                    for (name, func) in i.functions.iter() {
                        print_function(name, func, 2)
                    }
                    println!("  }}")
                }
            }
        }
        let val = component.call("hello-world", &[]);
        println!("Hello from the guest: {:?}", val);
    }
}

fn print_function(
    name: &str,
    func: &bindings::component::dyna::dynamic_component::Function,
    offset: usize,
) {
    let params = func
        .params
        .iter()
        .map(|(name, typ)| format!("{name}: {:?}", typ.kind()))
        .collect::<Vec<_>>()
        .join(", ");
    let result = func.result.kind();
    println!(
        "{}{name}: func({}) -> {:?};",
        "  ".repeat(offset),
        params,
        result
    );
}
