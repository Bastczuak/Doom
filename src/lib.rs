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
use crate::utils::{set_panic_hook, to_vec_u8};
use crate::wad::Wad;
use specs::prelude::*;
use wasm_bindgen::prelude::*;
use crate::system::keyboard::Keyboard;
use crate::component::{MovementCommand, Direction, Position, Velocity, KeyboardControlled};
use crate::system::physics::Physics;

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
    let mut world = World::new();
    world.register::<KeyboardControlled>();
    world.register::<Position>();
    world.register::<Velocity>();
    let movement_command: Option<MovementCommand> = None;
    world.insert(movement_command);
    Ok(Doom { wad, world })
  }

  #[wasm_bindgen]
  pub fn tick(&mut self, events: &str) -> Result<JsValue, JsValue> {
    let mut movement_command : Option<MovementCommand> = None;
    match events {
      "a" => {
        movement_command = Some(MovementCommand::Move(Direction::Left))
      }
      "d" => {
        movement_command = Some(MovementCommand::Move(Direction::Right))
      }
      "w" => {
        movement_command = Some(MovementCommand::Move(Direction::Up))
      }
      "s" => {
        movement_command = Some(MovementCommand::Move(Direction::Down))
      }
      _ => {
        movement_command = Some(MovementCommand::Stop)
      }
    }
    *self.world.write_resource() = movement_command;
    self.run_systems();

    let position_storage = self.world.read_storage::<Position>();
    let positions: Vec<&Position> = position_storage.join().collect();
    Ok(JsValue::from_serde(&positions).unwrap())
  }

  fn run_systems(&mut self) {
    let mut keyboard = Keyboard {};
    keyboard.run_now(&self.world);
    let mut physics = Physics {};
    physics.run_now(&self.world);
    self.world.maintain();
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
  ) -> Result<(), JsValue> {
    create_player(map, id, &self.wad, &mut self.world).map_err(|e| e.to_string())?;
    Ok(())
  }
}
