use crate::datatypes::Result;
use crate::player::Player;
use crate::wad::linedef::LineDef;
use crate::wad::node::Node;
use crate::wad::thing::Thing;
use crate::wad::vertex::Vertex;
use serde::{Deserialize, Serialize};

pub trait MapMetaData: Sized {
  fn read(wad: &[u8], offset: usize) -> Result<Self>;
  fn lump_name() -> String;
  fn size_in_bytes() -> u32;
  fn index() -> usize;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
  name: String,
  vertexes: Vec<Vertex>,
  line_defs: Vec<LineDef>,
  things: Vec<Thing>,
  nodes: Vec<Node>,
  player: Player,
  x_min: i16,
  x_max: i16,
  y_min: i16,
  y_max: i16,
}

impl Map {
  pub fn new(
    name: &str,
    vertexes: Vec<Vertex>,
    line_defs: Vec<LineDef>,
    things: Vec<Thing>,
    nodes: Vec<Node>,
    player: Player,
  ) -> Self {
    let mut map = Map {
      name: String::from(name),
      vertexes,
      line_defs,
      things,
      nodes,
      player,
      x_min: std::i16::MAX,
      x_max: std::i16::MIN,
      y_min: std::i16::MAX,
      y_max: std::i16::MIN,
    };
    map.calc_map_shift();
    map.calc_player_position();
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

  fn calc_player_position(&mut self) {
    for thing in &self.things {
      if thing.typ == self.player.id {
        self.player.x = thing.x;
        self.player.y = thing.y;
        self.player.direction = thing.direction
      }
    }
  }
}
