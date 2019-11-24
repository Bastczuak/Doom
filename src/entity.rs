use crate::component::map::Map;
use crate::datatypes::Result;
use crate::errors::DoomError;
use crate::wad::linedef::LineDef;
use crate::wad::vertex::Vertex;
use crate::wad::Wad;
use specs::{Builder, Entity, World, WorldExt};

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
