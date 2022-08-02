use bracket_lib::prelude::Point;
use specs::prelude::Entity;

pub enum Input {
    Left, Right, Up, Down,
}

pub enum Action {
    Nil,
    Move(Entity, Point),
}
