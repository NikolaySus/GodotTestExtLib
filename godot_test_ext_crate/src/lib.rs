use godot::prelude::*;

struct GodotTestExt;

#[gdextension]
unsafe impl ExtensionLibrary for GodotTestExt {}

use godot::engine::Node2D;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Test {
    node2d: Base<Node2D>
}

use godot::engine::INode2D;

#[godot_api]
impl INode2D for Test {
    fn init(node2d: Base<Node2D>) -> Self {
        godot_print!("Hello!");
        Self { node2d }
    }

    fn process(&mut self, _delta:f64) {
        godot_print!("Hello from process!");
    }
}
