use crate::datatypes::Result;
use crate::errors::DoomError;
use crate::map::{LineDef, Map, MapMetaData, Things, Vertex};
use crate::player::Player;
use crate::utils;

#[derive(Debug)]
pub struct Header {
  pub wad_type: String,
  pub directory_count: u32,
  pub directory_offset: u32,
}

#[derive(Debug)]
pub struct Directory {
  pub lump_name: String,
  pub lump_size: u32,
  pub lump_offset: u32,
}

pub struct Wad {
  wad: Vec<u8>,
  directories: Vec<Directory>,
}

impl Wad {
  pub fn new(wad: &[u8]) -> std::io::Result<Self> {
    Ok(Wad {
      wad: Vec::from(wad),
      directories: Vec::new(),
    })
  }

  pub fn read_wad(&mut self) -> Result<()> {
    let header = self.read_header(0)?;
    for dir in 0..header.directory_count {
      let directory = self.read_directory((header.directory_offset + dir * 16) as usize)?;
      self.directories.push(directory);
    }
    Ok(())
  }

  pub fn read_map(&self, name: &str) -> Result<Map> {
    match self.find_map_index(name) {
      Some(mut map_index) => {
        let vertexes: Vec<Vertex> = self.read_map_data_for::<Vertex>(map_index)?;
        let line_defs: Vec<LineDef> = self.read_map_data_for::<LineDef>(map_index)?;
        let things: Vec<Things> = self.read_map_data_for::<Things>(map_index)?;
        let player = Player::new(1);
        Ok(Map::new(name, vertexes, line_defs, things, player))
      }

      None => Err(DoomError::Wad(format!(
        "Failed to load MAP: {}",
        name
      ))),
    }
  }

  fn read_header(&self, offset: usize) -> Result<Header> {
    let wad_type = String::from_utf8(Vec::from(&self.wad[offset..offset + 4]))?
      .trim_matches(char::from(0))
      .to_string();
    let directory_count = utils::to_u32(&self.wad, offset + 4)?;
    let directory_offset = utils::to_u32(&self.wad, offset + 8)?;
    Ok(Header {
      wad_type,
      directory_count,
      directory_offset,
    })
  }

  fn read_directory(&self, offset: usize) -> Result<Directory> {
    let lump_offset = utils::to_u32(&self.wad, offset)?;
    let lump_size = utils::to_u32(&self.wad, offset + 4)?;
    let lump_name = String::from_utf8(Vec::from(&self.wad[offset + 8..offset + 16]))?
      .trim_matches(char::from(0))
      .to_string();
    Ok(Directory {
      lump_name,
      lump_size,
      lump_offset,
    })
  }

  fn find_map_index(&self, name: &str) -> Option<usize> {
    for i in 0..self.directories.len() {
      if self.directories[i].lump_name == String::from(name) {
        return Some(i);
      }
    }
    None
  }

  fn read_map_data_for<T: MapMetaData>(&self, map_index: usize) -> Result<Vec<T>> {
    let mut index = map_index + T::index();

    if self.directories[index].lump_name != T::lump_name() {
      return Err(DoomError::Wad(format!(
        "Failed to load {} for MAP {}",
        T::lump_name(),
        self.directories[map_index].lump_name,
      )));
    }

    let mut vec = Vec::new();
    for i in 0..self.directories[index].lump_size / T::size_in_bytes() {
      let vertex = MapMetaData::read(
        &self.wad,
        (self.directories[index].lump_offset + i * T::size_in_bytes()) as usize,
      )?;
      vec.push(vertex);
    }
    Ok(vec)
  }
}
