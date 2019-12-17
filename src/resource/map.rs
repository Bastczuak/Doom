use crate::resource::linedef::LineDef;
use crate::resource::vertex::Vertex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
  vertexes: Vec<Vertex>,
  line_defs: Vec<LineDef>,
  x_min: i16,
  x_max: i16,
  y_min: i16,
  y_max: i16,
}

impl Map {
  pub fn new(vertexes: Vec<Vertex>, line_defs: Vec<LineDef>) -> Self {
    let mut map = Map {
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
