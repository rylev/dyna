#[allow(dead_code)]
mod bindings;

use bindings::component::dyna::dynamic_component::load_component;
use bindings::Guest;

pub struct Component;

impl Guest for Component {
    fn hello() {
        let component = load_component("example/static_guest.wasm");
        let val = component.call("hello-world", &[]);
        println!("Hello from the guest: {:?}", val);
    }
}
