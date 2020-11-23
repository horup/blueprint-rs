use Default;

use crate::{event::Event, world::{GameWorld, World}};


#[derive(Debug, Copy, Clone, Default)]
pub struct Keyboard {
    pub left:bool,
    pub right:bool,
    pub up:bool,
    pub down:bool
}

// TODO: implement generics support
#[derive(Debug, Copy, Clone, Default)]
pub struct Input {
    pub keyboard:Keyboard
}

pub struct Context<'a, W:GameWorld> {
    pub world:&'a mut World<W>,
    pub event:&'a Event<W::Event>,
    pub input:Input
}