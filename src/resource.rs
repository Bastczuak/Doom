use crate::datatypes::Result;
use crate::errors::DoomError;
use crate::map::Map;
use crate::wad::node::Node;
use crate::wad::seg::Seg;
use crate::wad::ssector::SSector;
use crate::wad::Wad;
use specs::World;
use crate::wad::vertex::{Vertex, VisibleVertexes};
use crate::wad::linedef::LineDef;

pub fn create_map(map: &str, wad: &Wad, ecs: &mut World) -> Result<Map> {
  match wad.find_map_index(map) {
    Some(map_index) => {
      let nodes: Vec<Node> = wad.read_wad_for(map_index)?;
      ecs.insert(nodes);
      let segs: Vec<Seg> = wad.read_wad_for(map_index)?;
      ecs.insert(segs);
      let ssector: Vec<SSector> = wad.read_wad_for(map_index)?;
      ecs.insert(ssector);
      let vertexes: Vec<Vertex> = wad.read_wad_for(map_index)?;
      ecs.insert(vertexes);
      let line_defs: Vec<LineDef> = wad.read_wad_for(map_index)?;
      ecs.insert(line_defs);
      let visible_vertexes :VisibleVertexes = Default::default();
      ecs.insert(visible_vertexes);
      // TODO: I create vertexes and linedefs two times because of borrowing issues.
      Ok(Map::new(
        wad.read_wad_for::<Vertex>(map_index)?,
        wad.read_wad_for::<LineDef>(map_index)?,
      ))
    }

    None => Err(DoomError::Wad(format!("Failed to load MAP: {}", map))),
  }
}
