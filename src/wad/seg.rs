use crate::datatypes::MapLumpsIndex;
use crate::errors::DoomError;
use crate::utils::to_u16;
use crate::wad::WadMetaData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Seg {
  start_vertex: u16, // Vertex 1
  end_vertex: u16,   // Vertex 2
  angle: u16,
  linedef: u16,
  side: u16, // 0 = front, 1 = back
  offset: u16,
}

impl WadMetaData for Seg {
  fn read(wad: &[u8], offset: usize) -> Result<Self, DoomError> {
    let start_vertex = to_u16(wad, offset)?;
    let end_vertex = to_u16(wad, offset + 2)?;
    let angle = to_u16(wad, offset + 4)?;
    let linedef = to_u16(wad, offset + 6)?;
    let side = to_u16(wad, offset + 8)?;
    let offset = to_u16(wad, offset + 10)?;
    Ok(Seg {
      start_vertex,
      end_vertex,
      angle,
      linedef,
      side,
      offset,
    })
  }

  fn lump_name() -> String {
    String::from("SEGS")
  }

  fn size_in_bytes() -> u32 {
    std::mem::size_of::<Seg>() as u32
  }

  fn index() -> usize {
    MapLumpsIndex::SEGS as usize
  }
}
