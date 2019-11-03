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
  name: String,
  vertexes: Vec<Vertex>,
  line_defs: Vec<LineDef>,
  x_min: i16,
  x_max: i16,
  y_min: i16,
  y_max: i16,
}

impl Map {
  pub fn new(name: &str, vertexes: Vec<Vertex>, line_defs: Vec<LineDef>) -> Self {
    let mut map = Map {
      name: String::from(name),
      vertexes,
      line_defs,
      x_min: std::i16::MAX,
      x_max: std::i16::MIN,
      y_min: std::i16::MAX,
      y_max: std::i16::MIN,
    };
    map.calc_map_shift();
    map
  }

  fn calc_map_shift(&mut self) {
    for vertex in &self.vertexes {
      if self.x_min > vertex.x {
        self.x_min = vertex.x;
      } else if self.x_max < vertex.x {
        self.x_max = vertex.x;
      }

      if self.y_min > vertex.y {
        self.y_min = vertex.y;
      } else if self.y_max < vertex.y {
        self.y_max = vertex.y;
      }
    }
  }
}
