use std::marker::PhantomData;

use specs_derive::Component;
use specs::prelude::*;
use bracket_lib::prelude::*;

#[derive(Component)]
pub struct Pos(pub Point);

#[derive(Component)]
pub struct Draw {
    pub tile: String,
}

#[derive(Component)]
pub struct Player {}


impl Pos {
    pub fn new(x: i32, y: i32) -> Pos {
        Pos(Point{x,y})
    }
}

pub struct Handle<T> {
    pub index: usize,
    phantom: PhantomData<T>,
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle{index: self.index, phantom: PhantomData}
    }
}

impl<T> Copy for Handle<T> {}
