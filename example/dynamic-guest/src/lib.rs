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
        let val = component.call("hello-world", &[]);
        println!("Hello from the guest: {:?}", val);
    }
}
