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

#[wasm_bindgen]
pub fn init(downloaded_wad: &JsValue, map_name: &str) -> Result<JsValue, JsValue> {
  set_panic_hook();
  let buffer = to_vec_u8(downloaded_wad);
  let mut wad = Wad::new(&buffer).map_err(|e| e.to_string())?;
  wad.read_wad().map_err(|e| e.to_string())?;
  let map = wad.read_map(map_name).map_err(|e| e.to_string())?;
  Ok(JsValue::from_serde(&map).unwrap())
}
