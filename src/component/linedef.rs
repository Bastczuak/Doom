use crate::datatypes::{MapLumpsIndex, Result};
use crate::utils::to_u16;
use crate::wad::WadMetaData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LineDef {
  pub start_vertex: u16,
  pub end_vertex: u16,
  pub flags: u16,
  pub line_type: u16,
  pub sector_tag: u16,
  pub front_side_def: u16,
  pub back_side_def: u16,
}

impl WadMetaData for LineDef {
  fn read(wad: &[u8], offset: usize) -> Result<Self> {
    let start_vertex = to_u16(wad, offset)?;
    let end_vertex = to_u16(wad, offset + 2)?;
    let flags = to_u16(wad, offset + 4)?;
    let line_type = to_u16(wad, offset + 6)?;
    let sector_tag = to_u16(wad, offset + 8)?;
    let front_side_def = to_u16(wad, offset + 10)?;
    let back_side_def = to_u16(wad, offset + 12)?;
    Ok(LineDef {
      start_vertex,
      end_vertex,
      flags,
      line_type,
      sector_tag,
      front_side_def,
      back_side_def,
    })
  }

  fn lump_name() -> String {
    String::from("LINEDEFS")
  }

  fn size_in_bytes() -> u32 {
    std::mem::size_of::<LineDef>() as u32
  }

  fn index() -> usize {
    MapLumpsIndex::LINEDEFS as usize
  }
}
