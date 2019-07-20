use crate::shapes::Shape;
use specs::prelude::*;

#[derive(Debug)]
pub struct Vel(pub f32, pub f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Pos(pub f32, pub f32);

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Geometry(pub Shape);

impl Component for Geometry {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

#[derive(Debug)]
/// R G B A
pub struct Color(pub (u8, u8, u8, u8));

impl Component for Color {
    type Storage = VecStorage<Self>;
}
