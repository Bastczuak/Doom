mod component;
mod datatypes;
mod entity;
mod errors;
mod player;
mod system;
mod utils;
mod wad;

use crate::component::map::Map;
use crate::entity::create_map;
use crate::system::map::MapSystem;
use crate::utils::{set_panic_hook, to_vec_u8};
use crate::wad::Wad;
use specs::{RunNow, World, WorldExt};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const SUB_SECTOR_IDENTIFIER: usize = 0x8000;

#[wasm_bindgen(js_name = "checkForSubSector")]
pub fn check_for_sub_sector(node: usize) -> bool {
  node & SUB_SECTOR_IDENTIFIER > 0
}

#[wasm_bindgen]
pub struct Doom {
  wad: Wad,
}

#[wasm_bindgen]
impl Doom {
  pub fn new(downloaded_wad: &JsValue) -> Result<Doom, JsValue> {
    set_panic_hook();
    let buffer = to_vec_u8(downloaded_wad);
    let wad = Wad::new(&buffer).map_err(|e| e.to_string())?;
    Ok(Doom { wad })
  }

  pub fn load(&self, map: &str, js_callback: js_sys::Function) {
    let mut world = specs::World::new();
    create_map(map, &self.wad, &mut world);
    let mut map = MapSystem::new(js_callback);
    map.run_now(&world);
    world.maintain();
  }
}
