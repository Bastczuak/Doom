use crate::datatypes::{MapLumpsIndex, Result};
use crate::utils::{to_i16, to_u16};
use crate::wad::WadMetaData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
  x_partition: i16,
  y_partition: i16,
  change_x_partition: i16,
  change_y_partition: i16,
  right_box_top: i16,
  right_box_bottom: i16,
  right_box_left: i16,
  right_box_right: i16,
  left_box_top: i16,
  left_box_bottom: i16,
  left_box_left: i16,
  left_box_right: i16,
  right_child: u16, // front
  left_child: u16,  // back
}

impl WadMetaData for Node {
  fn read(wad: &[u8], offset: usize) -> Result<Self> {
    let x_partition = to_i16(wad, offset)?;
    let y_partition = to_i16(wad, offset + 2)?;
    let change_x_partition = to_i16(wad, offset + 4)?;
    let change_y_partition = to_i16(wad, offset + 6)?;
    let right_box_top = to_i16(wad, offset + 8)?;
    let right_box_bottom = to_i16(wad, offset + 10)?;
    let right_box_left = to_i16(wad, offset + 12)?;
    let right_box_right = to_i16(wad, offset + 14)?;
    let left_box_top = to_i16(wad, offset + 16)?;
    let left_box_bottom = to_i16(wad, offset + 18)?;
    let left_box_left = to_i16(wad, offset + 20)?;
    let left_box_right = to_i16(wad, offset + 22)?;
    let right_child = to_u16(wad, offset + 24)?;
    let left_child = to_u16(wad, offset + 26)?;
    Ok(Node {
      x_partition,
      y_partition,
      change_x_partition,
      change_y_partition,
      right_box_top,
      right_box_bottom,
      right_box_left,
      right_box_right,
      left_box_top,
      left_box_bottom,
      left_box_left,
      left_box_right,
      right_child,
      left_child,
    })
  }

  fn lump_name() -> String {
    String::from("NODES")
  }

  fn size_in_bytes() -> u32 {
    std::mem::size_of::<Node>() as u32
  }

  fn index() -> usize {
    MapLumpsIndex::NODES as usize
  }
}
