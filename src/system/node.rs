use crate::component::node::Node;
use specs::{ReadStorage, System};
use wasm_bindgen::JsValue;

pub struct NodeSystem {
  js_callback: js_sys::Function,
}

impl NodeSystem {
  pub fn new(js_callback: js_sys::Function) -> Self {
    NodeSystem { js_callback }
  }
}

impl<'a> System<'a> for NodeSystem {
  type SystemData = ReadStorage<'a, Node>;

  fn run(&mut self, nodes: Self::SystemData) {
    use specs::Join;

    let nodes: Vec<&Node> = nodes.join().collect();

    let this = JsValue::NULL;
    let nodes = JsValue::from_serde(&nodes).unwrap();
    self.js_callback.call1(&this, &nodes).unwrap();
  }
}
