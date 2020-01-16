use crate::datatypes::MapLumpsIndex;
use crate::errors::DoomError;
use crate::utils::to_u16;
use crate::wad::WadMetaData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SSector {
  seg_count: u16,
  first_seg: u16,
}

impl WadMetaData for SSector {
  fn read(wad: &[u8], offset: usize) -> Result<Self, DoomError> {
    let seg_count = to_u16(wad, offset)?;
    let first_seg = to_u16(wad, offset + 2)?;
    Ok(SSector {
      seg_count,
      first_seg,
    })
  }

  fn lump_name() -> String {
    String::from("SSECTORS")
  }

  fn size_in_bytes() -> u32 {
    std::mem::size_of::<SSector>() as u32
  }

  fn index() -> usize {
    MapLumpsIndex::SSECTORS as usize
  }
}
