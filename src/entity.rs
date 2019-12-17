use crate::component::linedef::LineDef;
use crate::component::map::Map;
use crate::component::node::Node;
use crate::component::thing::Thing;
use crate::component::vertex::Vertex;
use crate::datatypes::Result;
use crate::errors::DoomError;
use crate::wad::Wad;
use specs::{Builder, World, WorldExt};

pub fn create_map(map: &str, wad: &Wad, world: &mut World) -> Result<()> {
  match wad.find_map_index(map) {
    Some(map_index) => {
      world.register::<Map>();
      let vertexes = wad.read_wad_for::<Vertex>(map_index)?;
      let line_defs = wad.read_wad_for::<LineDef>(map_index)?;
      let map = Map::new(vertexes, line_defs);
      world.create_entity().with(map).build();
      Ok(())
    }

    None => Err(DoomError::Wad(format!("Failed to load MAP: {}", map))),
  }
}

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

    None => Err(DoomError::Wad(format!("Failed to load MAP: {}", map))),
  }
}

pub fn create_nodes(map: &str, wad: &Wad, world: &mut World) -> Result<()> {
  match wad.find_map_index(map) {
    Some(map_index) => {
      world.register::<Node>();
      let nodes = wad.read_wad_for::<Node>(map_index)?;
      for node in nodes {
        world.create_entity().with(node).build();
      }
      Ok(())
    }
    None => Err(DoomError::Wad(format!("Failed to load MAP: {}", map))),
  }
}
