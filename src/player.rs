use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
  pub id: u16,
  pub x: i16,
  pub y: i16,
  pub direction: u16,
}

impl Player {
  pub fn new(id: u16) -> Self {
    Player {
      id,
      x: 0,
      y: 0,
      direction: 0,
    }
  }
}
