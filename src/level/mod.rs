use crate::components::*;
use num_derive::FromPrimitive;
use specs::world::WorldExt;
use specs::World;

pub mod common;
pub mod level1;

pub trait Level {
    fn load_wasm(&mut self, wasm_bytes: &[u8]) -> Result<(), String>;
    // TODO: add timestep here
    fn step_time(&mut self);
    fn borrow_world(&self) -> &World;
    /// resets the state of the level, as if it was just created
    fn reset(&mut self);
    //fn get_events(&mut self) -> Vec<LevelEvent>;
}

/*#[derive(Debug)]
pub enum LevelEvent {
    WinConditionReached,
    LoseConditionReached,
}*/

#[derive(Debug, FromPrimitive, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Direction {
    Up = 0,
    Down,
    Left,
    Right,
}

/// These are the messages that the Wasm can send to the Level
#[derive(Debug, Clone)]
pub enum LevelMessage {
    PlayerMoveRequest { direction: Direction },
}

#[derive(Debug, Default)]
pub struct LevelMessages(pub Vec<LevelMessage>);

pub fn register_rendering_components(world: &mut World) {
    world.register::<Pos>();
    world.register::<Geometry>();
    world.register::<Color>();
    world.register::<Player>();
}
