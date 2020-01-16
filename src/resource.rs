use crate::datatypes::Result;
use crate::errors::DoomError;
use crate::wad::Wad;
use crate::map::Map;

pub fn create_map(map: &str, wad: &Wad) -> Result<Map> {
  match wad.find_map_index(map) {
    Some(map_index) => {
      let vertexes = wad.read_wad_for(map_index)?;
      let line_defs = wad.read_wad_for(map_index)?;
      let map = Map::new(vertexes, line_defs);
      Ok(map)
    }

    None => Err(DoomError::Wad(format!("Failed to load MAP: {}", map))),
  }
}
