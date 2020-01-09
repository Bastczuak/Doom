pub(crate) mod angle;
pub(crate) mod linedef;
pub(crate) mod map;
pub(crate) mod node;
pub(crate) mod seg;
pub(crate) mod ssector;
pub(crate) mod vertex;
pub(crate) mod thing;

use crate::datatypes::Result;
use crate::errors::DoomError;
use crate::resource::map::Map;
use crate::wad::Wad;

pub fn create_map(map: &str, wad: &Wad) -> Result<Map> {
  match wad.find_map_index(map) {
    Some(map_index) => {
      let vertexes = wad.read_wad_for(map_index)?;
      let line_defs = wad.read_wad_for(map_index)?;
      let nodes = wad.read_wad_for(map_index)?;
      let segs = wad.read_wad_for(map_index)?;
      let ssector = wad.read_wad_for(map_index)?;
      let map = Map::new(vertexes, line_defs, nodes, segs, ssector);
      Ok(map)
    }

    None => Err(DoomError::Wad(format!("Failed to load MAP: {}", map))),
  }
}
