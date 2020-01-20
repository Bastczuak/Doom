use crate::component::*;
use specs::prelude::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
  type SystemData = (
    WriteStorage<'a, Position>,
    WriteStorage<'a, Rotation>,
    ReadStorage<'a, Velocity>,
  );

  fn run(&mut self, mut data: Self::SystemData) {
    use self::Direction::*;
    for (pos, rot, vel) in (&mut data.0, &mut data.1, &data.2).join() {
      let (x, y) = match vel.direction {
        Left => (-vel.speed, 0),
        Right => (vel.speed, 0),
        Down => (0, -vel.speed),
        Up => (0, vel.speed),
      };
      pos.x += x;
      pos.y += y;
      rot.angle += rot.speed;
    }
  }
}
