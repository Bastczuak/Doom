use crate::angle::Angle;
use crate::component::{Direction, KeyboardControlled, Position, Rotation, Velocity};
use crate::datatypes::Result;
use crate::errors::DoomError;
use crate::wad::thing::Thing;
use crate::wad::Wad;
use specs::{Builder, World, WorldExt};

pub fn create_player(map: &str, id: u16, wad: &Wad, world: &mut World) -> Result<()> {
  match wad.find_map_index(map) {
    Some(map_index) => {
      let things = wad.read_wad_for::<Thing>(map_index)?;
      match things.iter().find(|&thing| thing.typ == id) {
        Some(thing) => {
          world
            .create_entity()
            .with(KeyboardControlled)
            .with(Position {
              x: thing.x,
              y: thing.y,
            })
            .with(Velocity {
              speed: 0,
              direction: Direction::Right,
            })
            .with(Rotation {
              speed: 0.0,
              angle: Angle::new(thing.direction as f32),
              fov: 90,
            })
            .build();
          Ok(())
        }
        None => Err(DoomError::Wad(format!("Failed to load THINGS: {}", id))),
      }
    }
    None => Err(DoomError::Wad(format!("Failed to load THINGS: {}", id))),
  }
}
