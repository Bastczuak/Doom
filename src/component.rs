use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

pub enum MovementCommand {
  Move(Direction),
  Stop,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

#[derive(Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Position {
  pub x: i16,
  pub y: i16,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Velocity {
  pub speed: i16,
  pub direction: Direction,
}
