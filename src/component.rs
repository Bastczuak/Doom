use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::Component;
use crate::angle::Angle;

#[derive(Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
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

pub enum RotationCommand {
  Rotate(Direction),
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

#[derive(Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Rotation {
  pub speed: f32,
  pub angle: Angle,
  pub fov: i16,
}
