use crate::component::thing::Thing;
use crate::datatypes::Result;
use crate::errors::DoomError;
use crate::wad::Wad;
use specs::{Builder, World, WorldExt};

pub fn create_player(map: &str, id: u16, wad: &Wad, world: &mut World) -> Result<()> {
  match wad.find_map_index(map) {
    Some(map_index) => {
      world.register::<Thing>();
      let things = wad.read_wad_for::<Thing>(map_index)?;
      for thing in things {
        if thing.typ == id {
          world.create_entity().with(thing).build();
        }
      }
      Ok(())
    }

    None => Err(DoomError::Wad(format!("Failed to load THINGS: {}", id))),
  }
}
