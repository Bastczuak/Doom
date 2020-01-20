mod component;
mod datatypes;
mod entity;
mod errors;
mod map;
mod resource;
mod system;
mod utils;
mod wad;
mod angle;

use crate::component::*;
use crate::entity::create_player;
use crate::resource::create_map;
use crate::system::keyboard::Keyboard;
use crate::system::physics::Physics;
use crate::utils::{set_panic_hook, to_vec_u8};
use crate::wad::Wad;
use specs::prelude::*;
use wasm_bindgen::prelude::*;
use crate::wad::node::Node;
use crate::wad::seg::Seg;
use crate::wad::ssector::SSector;

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

// TODO: everytime the player moves we uopdate which segs should be rendered
// But this is handled at first in javascript. The Physics system calculate the
// angle with every tick. Javascript gets it from rust and calclates the segs.

#[wasm_bindgen]
pub struct Doom {
  wad: Wad,
  ecs: World,
}

#[wasm_bindgen]
impl Doom {
  pub fn new(downloaded_wad: &JsValue) -> Result<Doom, JsValue> {
    set_panic_hook();
    let buffer = to_vec_u8(downloaded_wad);
    let wad = Wad::new(&buffer).map_err(|e| e.to_string())?;
    let mut ecs = World::new();
    ecs.register::<KeyboardControlled>();
    ecs.register::<Position>();
    ecs.register::<Velocity>();
    ecs.register::<Rotation>();
    Ok(Doom { wad, ecs })
  }

  #[wasm_bindgen]
  pub fn tick(&mut self, events: &str) {
    match events {
      "a" => *self.ecs.write_resource() = Some(MovementCommand::Move(Direction::Left)),
      "d" => *self.ecs.write_resource() = Some(MovementCommand::Move(Direction::Right)),
      "w" => *self.ecs.write_resource() = Some(MovementCommand::Move(Direction::Up)),
      "s" => *self.ecs.write_resource() = Some(MovementCommand::Move(Direction::Down)),
      "q" => *self.ecs.write_resource() = Some(RotationCommand::Rotate(Direction::Left)),
      "e" => *self.ecs.write_resource() = Some(RotationCommand::Rotate(Direction::Right)),
      _ => {
        *self.ecs.write_resource() = Some(MovementCommand::Stop);
        *self.ecs.write_resource() = Some(RotationCommand::Stop);
      }
    }
    self.run_systems();
  }

  pub fn get_player(&self) -> Result<JsValue, JsValue> {
    let position_storage = self.ecs.read_storage::<Position>();
    let rotation_storage = self.ecs.read_storage::<Rotation>();
    let joined: Vec<_> = (&position_storage, &rotation_storage).join().collect();
    Ok(JsValue::from_serde(&joined).unwrap())
  }

  pub fn get_nodes(&self) -> Result<JsValue, JsValue> {
    let nodes = &*self.ecs.read_resource::<Vec<Node>>();
    Ok(JsValue::from_serde(&nodes).unwrap())
  }

  pub fn get_segs(&self) -> Result<JsValue, JsValue> {
    let segs = &*self.ecs.read_resource::<Vec<Seg>>();
    Ok(JsValue::from_serde(&segs).unwrap())
  }

  pub fn get_ssecttors(&self) -> Result<JsValue, JsValue> {
    let ssectors_resource = &*self.ecs.read_resource::<Vec<SSector>>();
    Ok(JsValue::from_serde(&ssectors_resource).unwrap())
  }

  fn run_systems(&mut self) {
    let mut keyboard = Keyboard {};
    keyboard.run_now(&self.ecs);
    let mut physics = Physics {};
    physics.run_now(&self.ecs);
    self.ecs.maintain();
  }

  #[wasm_bindgen(js_name = "loadMap")]
  pub fn load_map(&mut self, map: &str) -> Result<JsValue, JsValue> {
    let map = create_map(map, &self.wad, &mut self.ecs).map_err(|e| e.to_string())?;
    let js_value = JsValue::from_serde(&map).unwrap();
    self.ecs.insert(map);
    Ok(js_value)
  }

  #[wasm_bindgen(js_name = "loadPlayer")]
  pub fn load_player(&mut self, map: &str, id: u16) -> Result<(), JsValue> {
    let movement_command: Option<MovementCommand> = None;
    let rotation_commnad: Option<RotationCommand> = None;
    self.ecs.insert(movement_command);
    self.ecs.insert(rotation_commnad);
    create_player(map, id, &self.wad, &mut self.ecs).map_err(|e| e.to_string())?;
    Ok(())
  }
}
