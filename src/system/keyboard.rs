use crate::component::*;
use specs::prelude::*;

const PLAYER_MOVEMENT_SPEED: i16 = 20;
const PLAYER_ROTATION_SPEED: f32 = 0.1875 * 20.0;

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
  type SystemData = (
    ReadExpect<'a, Option<MovementCommand>>,
    ReadExpect<'a, Option<RotationCommand>>,
    ReadStorage<'a, KeyboardControlled>,
    WriteStorage<'a, Velocity>,
    WriteStorage<'a, Rotation>,
  );

  fn run(&mut self, mut data: Self::SystemData) {
    let movement_command = match &*data.0 {
      Some(movement) => movement,
      None => return,
    };
    let rotation_command = match &*data.1 {
      Some(rotation) => rotation,
      None => return,
    };

    for (_, vel, rot) in (&data.2, &mut data.3, &mut data.4).join() {
      match movement_command {
        &MovementCommand::Move(direction) => {
          vel.speed = PLAYER_MOVEMENT_SPEED;
          vel.direction = direction;
        }
        MovementCommand::Stop => vel.speed = 0,
      }

      match rotation_command {
        &RotationCommand::Rotate(direction) => {
          match direction {
            Direction::Left => rot.speed = PLAYER_ROTATION_SPEED,
            Direction::Right => rot.speed = -PLAYER_ROTATION_SPEED,
            _ => rot.speed = 0.0,
          }
        }
        RotationCommand::Stop => rot.speed = 0.0,
      }
    }
  }
}
