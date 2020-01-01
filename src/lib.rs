mod component;
mod datatypes;
mod entity;
mod errors;
mod resource;
mod system;
mod utils;
mod wad;

use crate::entity::create_player;
use crate::resource::create_map;
use crate::system::thing::ThingsSystem;
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

#[wasm_bindgen(js_name = "getSubSector")]
pub fn get_sub_sector(node: usize) -> usize {
  node & !SUB_SECTOR_IDENTIFIER
}

#[wasm_bindgen]
pub struct Doom {
  wad: Wad,
  world: World,
}

#[wasm_bindgen]
impl Doom {
  pub fn new(downloaded_wad: &JsValue) -> Result<Doom, JsValue> {
    set_panic_hook();
    let buffer = to_vec_u8(downloaded_wad);
    let wad = Wad::new(&buffer).map_err(|e| e.to_string())?;
    let world = World::new();
    Ok(Doom { wad, world })
  }

  #[wasm_bindgen(js_name = "loadMap")]
  pub fn load_map(&mut self, map: &str) -> Result<JsValue, JsValue> {
    let map = create_map(map, &self.wad).map_err(|e| e.to_string())?;
    let js_value = JsValue::from_serde(&map).unwrap();
    self.world.insert(map);
    Ok(js_value)
  }

  #[wasm_bindgen(js_name = "loadPlayer")]
  pub fn load_player(
    &mut self,
    map: &str,
    id: u16,
    js_callback: js_sys::Function,
  ) -> Result<(), JsValue> {
    create_player(map, id, &self.wad, &mut self.world).map_err(|e| e.to_string())?;
    let mut player = ThingsSystem::new(js_callback);
    player.run_now(&self.world);
    self.world.maintain();
    Ok(())
  }
}
