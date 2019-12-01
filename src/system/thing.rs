use crate::component::thing::Thing;
use specs::{Entities, ReadStorage, System};
use wasm_bindgen::JsValue;

pub struct ThingsSystem {
  js_callback: js_sys::Function,
}

impl ThingsSystem {
  pub fn new(js_callback: js_sys::Function) -> Self {
    ThingsSystem { js_callback }
  }
}

impl<'a> System<'a> for ThingsSystem {
  type SystemData = ReadStorage<'a, Thing>;

  fn run(&mut self, things: Self::SystemData) {
    use specs::Join;

    for thing in things.join() {
      let this = JsValue::NULL;
      let x = JsValue::from_serde(&thing).unwrap();
      self.js_callback.call1(&this, &x).unwrap();
    }
  }
}
