mod bindings;

use bindings::{exports::component::example::foo, Guest};

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}

impl foo::Guest for Component {
    /// Say hello!
    fn hello_world() -> foo::Bar {
        foo::Bar { field: vec![42] }
    }
}
