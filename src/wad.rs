pub(crate) mod linedef;
pub(crate) mod map;
pub(crate) mod node;
pub(crate) mod seg;
pub(crate) mod ssector;
pub(crate) mod vertex;
pub(crate) mod thing;

use crate::datatypes::Result;
use crate::errors::DoomError;
use crate::utils;

pub trait WadMetaData: Sized {
  fn read(wad: &[u8], offset: usize) -> Result<Self>;
  fn lump_name() -> String;
  fn size_in_bytes() -> u32;
  fn index() -> usize;
}

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
  pub fn new(wad: &[u8]) -> Result<Wad> {
    let header = Wad::read_header(wad, 0)?;
    let mut directories = Vec::new();
    for dir in 0..header.directory_count {
      let directory = Wad::read_directory(wad, (header.directory_offset + dir * 16) as usize)?;
      directories.push(directory);
    }
    Ok(Wad {
      wad: Vec::from(wad),
      directories,
    })
  }

  fn read_header(wad: &[u8], offset: usize) -> Result<Header> {
    let wad_type = String::from_utf8(Vec::from(&wad[offset..offset + 4]))?
      .trim_matches(char::from(0))
      .to_string();
    let directory_count = utils::to_u32(&wad, offset + 4)?;
    let directory_offset = utils::to_u32(&wad, offset + 8)?;
    Ok(Header {
      wad_type,
      directory_count,
      directory_offset,
    })
  }

  fn read_directory(wad: &[u8], offset: usize) -> Result<Directory> {
    let lump_offset = utils::to_u32(&wad, offset)?;
    let lump_size = utils::to_u32(&wad, offset + 4)?;
    let lump_name = String::from_utf8(Vec::from(&wad[offset + 8..offset + 16]))?
      .trim_matches(char::from(0))
      .to_string();
    Ok(Directory {
      lump_name,
      lump_size,
      lump_offset,
    })
  }

  pub fn find_map_index(&self, name: &str) -> Option<usize> {
    for i in 0..self.directories.len() {
      if self.directories[i].lump_name == name {
        return Some(i);
      }
    }
    None
  }

  pub fn read_wad_for<T: WadMetaData>(&self, map_index: usize) -> Result<Vec<T>> {
    let index = map_index + T::index();

    if self.directories[index].lump_name != T::lump_name() {
      return Err(DoomError::Wad(format!(
        "Failed to load {} for MAP {}",
        T::lump_name(),
        self.directories[map_index].lump_name,
      )));
    }

    let mut vec = Vec::new();
    for i in 0..self.directories[index].lump_size / T::size_in_bytes() {
      let data = WadMetaData::read(
        &self.wad,
        (self.directories[index].lump_offset + i * T::size_in_bytes()) as usize,
      )?;
      vec.push(data);
    }
    Ok(vec)
  }
}
