use crate::component::map::Map;
use specs::{Entities, ReadStorage, System};
use wasm_bindgen::JsValue;

pub struct MapSystem {
  js_callback: js_sys::Function,
}

impl MapSystem {
  pub fn new(js_callback: js_sys::Function) -> Self {
    MapSystem { js_callback }
  }
}

impl<'a> System<'a> for MapSystem {
  type SystemData = ReadStorage<'a, Map>;

  fn run(&mut self, maps: Self::SystemData) {
    use specs::Join;

    for map in maps.join() {
      let this = JsValue::NULL;
      let x = JsValue::from_serde(&map).unwrap();
      self.js_callback.call1(&this, &x).unwrap();
    }
  }
}
