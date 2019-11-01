use crate::datatypes::{MapLumpsIndex, Result};
use crate::utils;

use serde::{Deserialize, Serialize};

pub trait MapMetaData: Sized {
  fn read(wad: &[u8], offset: usize) -> Result<Self>;
  fn lump_name() -> String;
  fn size_in_bytes() -> u32;
  fn index() -> usize;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vertex {
  pub x: i16,
  pub y: i16,
}

impl MapMetaData for Vertex {
  fn read(wad: &[u8], offset: usize) -> Result<Vertex> {
    let x = utils::to_i16(wad, offset)?;
    let y = utils::to_i16(wad, offset + 2)?;
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

impl MapMetaData for LineDef {
  fn read(wad: &[u8], offset: usize) -> Result<Self> {
    let start_vertex = utils::to_u16(wad, offset)?;
    let end_vertex = utils::to_u16(wad, offset + 2)?;
    let flags = utils::to_u16(wad, offset + 4)?;
    let line_type = utils::to_u16(wad, offset + 6)?;
    let sector_tag = utils::to_u16(wad, offset + 8)?;
    let front_side_def = utils::to_u16(wad, offset + 10)?;
    let back_side_def = utils::to_u16(wad, offset + 12)?;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
  pub name: String,
  pub vertexes: Vec<Vertex>,
  pub line_defs: Vec<LineDef>,
}
