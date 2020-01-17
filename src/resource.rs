use crate::datatypes::Result;
use crate::errors::DoomError;
use crate::map::Map;
use crate::wad::node::Node;
use crate::wad::seg::Seg;
use crate::wad::ssector::SSector;
use crate::wad::Wad;
use specs::{World, WorldExt};

pub fn create_map(map: &str, wad: &Wad, ecs: &mut World) -> Result<Map> {
  match wad.find_map_index(map) {
    Some(map_index) => {
      let nodes: Vec<Node> = wad.read_wad_for(map_index)?;
      let segs: Vec<Seg> = wad.read_wad_for(map_index)?;
      let ssector: Vec<SSector> = wad.read_wad_for(map_index)?;
      ecs.insert(nodes);
      ecs.insert(segs);
      ecs.insert(ssector);
      let vertexes = wad.read_wad_for(map_index)?;
      let line_defs = wad.read_wad_for(map_index)?;
      let map = Map::new(vertexes, line_defs);
      Ok(map)
    }

    None => Err(DoomError::Wad(format!("Failed to load MAP: {}", map))),
  }
}
