use crate::datatypes::{MapLumpsIndex, Result};
use crate::utils::to_i16;
use crate::wad::WadMetaData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VisibleVertexes {
  pub vertexes: Vec<(Vertex, Vertex)>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct Vertex {
  pub x: i16,
  pub y: i16,
}

impl WadMetaData for Vertex {
  fn read(wad: &[u8], offset: usize) -> Result<Vertex> {
    let x = to_i16(wad, offset)?;
    let y = to_i16(wad, offset + 2)?;
    Ok(Vertex { x, y })
  }

  fn lump_name() -> String {
    String::from("VERTEXES")
  }

  fn size_in_bytes() -> u32 {
    std::mem::size_of::<Vertex>() as u32
  }

  fn index() -> usize {
    MapLumpsIndex::VERTEXES as usize
  }
}
