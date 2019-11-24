mod datatypes;
mod errors;
mod player;
mod utils;
mod wad;

use crate::utils::{set_panic_hook, to_vec_u8};
use crate::wad::Wad;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = "doomInit")]
pub fn doom_init(downloaded_wad: &JsValue, map_name: &str) -> Result<JsValue, JsValue> {
  set_panic_hook();
  let buffer = to_vec_u8(downloaded_wad);
  let mut wad = Wad::new(&buffer).map_err(|e| e.to_string())?;
  wad.read_wad().map_err(|e| e.to_string())?;
  let map = wad.read_map(map_name).map_err(|e| e.to_string())?;
  Ok(JsValue::from_serde(&map).unwrap())
}

const SUB_SECTOR_IDENTIFIER: usize = 0x8000;

#[wasm_bindgen(js_name = "checkForSubSector")]
pub fn check_for_sub_sector(node: usize) -> bool {
  node & SUB_SECTOR_IDENTIFIER > 0
}

use specs::{Builder, Component, ReadStorage, RunNow, System, VecStorage, World, WorldExt};

#[derive(Debug)]
struct Position {
  x: f32,
  y: f32,
}

impl Component for Position {
  type Storage = VecStorage<Self>;
}

struct HelloWorld {
  callback: js_sys::Function,
}

impl<'a> System<'a> for HelloWorld {
  type SystemData = ReadStorage<'a, Position>;

  fn run(&mut self, position: Self::SystemData) {
    use specs::Join;
    let this = JsValue::NULL;
    for position in position.join() {
      let x = JsValue::from(format!("Hello, {:?}", &position).as_str());
      self.callback.call1(&this, &x).unwrap();
    }
  }
}

#[wasm_bindgen]
pub struct Doom {
  world: World,
  hello_world: HelloWorld
}

#[wasm_bindgen]
impl Doom {
  pub fn new(callback: js_sys::Function) -> Self {
    let mut world = World::new();
    world.register::<Position>();
    world
      .create_entity()
      .with(Position { x: 4.0, y: 7.0 })
      .build();

    world
      .create_entity()
      .with(Position { x: 8.0, y: 8.0 })
      .build();
    let mut hello_world = HelloWorld { callback };
    Doom {
      world,
      hello_world
    }
  }

  pub fn tick(&mut self) {
    self.hello_world.run_now(&self.world);
    self.world.maintain();
  }
}
