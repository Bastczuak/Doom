use crate::errors::DoomError;

pub enum MapLumpsIndex {
  THINGS = 1,
  LINEDEFS,
  SIDEDEFS,
  VERTEXES,
  SEAGS,
  SSECTORS,
  NODES,
  SECTORS,
  REJECT,
  BLOCKMAP,
  COUNT,
}

pub type Result<T> = std::result::Result<T, DoomError>;
