use crate::datatypes::{MapLumpsIndex, Result};
use crate::utils::{to_i16, to_u16};
use crate::wad::WadMetaData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Thing {
  pub x: i16,
  pub y: i16,
  pub direction: u16,
  pub typ: u16,
  pub flags: u16,
}

impl WadMetaData for Thing {
  fn read(wad: &[u8], offset: usize) -> Result<Self> {
    let x = to_i16(wad, offset)?;
    let y = to_i16(wad, offset + 2)?;
    let direction = to_u16(wad, offset + 4)?;
    let typ = to_u16(wad, offset + 6)?;
    let flags = to_u16(wad, offset + 8)?;
    Ok(Thing {
      x,
      y,
      direction,
      typ,
      flags,
    })
  }

  fn lump_name() -> String {
    String::from("THINGS")
  }

  fn size_in_bytes() -> u32 {
    std::mem::size_of::<Thing>() as u32
  }

  fn index() -> usize {
    MapLumpsIndex::THINGS as usize
  }
}
