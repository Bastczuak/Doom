pub(crate) mod linedef;
pub(crate) mod map;
pub(crate) mod node;
pub(crate) mod seg;
pub(crate) mod ssector;
pub(crate) mod vertex;

use crate::datatypes::Result;
use crate::errors::DoomError;
use crate::resource::map::Map;
use crate::resource::node::Node;
use crate::resource::seg::Seg;
use crate::resource::ssector::SSector;
use crate::wad::Wad;

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

pub fn create_nodes(map: &str, wad: &Wad) -> Result<Vec<Node>> {
  match wad.find_map_index(map) {
    Some(map_index) => {
      let nodes = wad.read_wad_for(map_index)?;
      Ok(nodes)
    }
    None => Err(DoomError::Wad(format!("Failed to load NODES: {}", map))),
  }
}

pub fn create_segs(map: &str, wad: &Wad) -> Result<Vec<Seg>> {
  match wad.find_map_index(map) {
    Some(map_index) => {
      let segs = wad.read_wad_for(map_index)?;
      Ok(segs)
    }
    None => Err(DoomError::Wad(format!("Failed to load Segs: {}", map))),
  }
}

pub fn create_ssectors(map: &str, wad: &Wad) -> Result<Vec<SSector>> {
  match wad.find_map_index(map) {
    Some(map_index) => {
      let ssector = wad.read_wad_for(map_index)?;
      Ok(ssector)
    }
    None => Err(DoomError::Wad(format!("Failed to load Segs: {}", map))),
  }
}
