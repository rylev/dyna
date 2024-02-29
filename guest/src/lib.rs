#[allow(dead_code)]
mod bindings;

use bindings::component::dyna::dynamic_component::load_component;
use bindings::Guest;

pub struct Component;

impl Guest for Component {
    fn hello() {
        let component =
            load_component("/Users/rylev/.cargo_target/wasm32-unknown-unknown/debug/example.wasm");
        component.call("hello-world");
    }
}
