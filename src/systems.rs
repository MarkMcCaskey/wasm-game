use crate::components::*;
use crate::level::*;
use specs::prelude::*;

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.0 += vel.0;
            pos.1 += vel.1;
        }
    }
}

pub struct PlayerInput;

impl<'a> System<'a> for PlayerInput {
    type SystemData = (
        Read<'a, LevelMessages>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (level_events, mut vels, _) = data;

        for level_event in &level_events.0 {
            for vel in (&mut vels).join() {
                match level_event {
                    LevelMessage::PlayerMoveRequest { direction } => {
                        let (x, y) = match direction {
                            Direction::Down => (0., -0.01),
                            Direction::Up => (0., 0.01),
                            Direction::Left => (-0.01, 0.),
                            Direction::Right => (0., 0.01),
                        };
                        vel.0 += x;
                        vel.1 += y;
                    }
                }
            }
        }
    }
}
