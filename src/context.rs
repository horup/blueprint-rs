use Default;

use crate::{event::Event, world::{GameWorld, World}};


#[derive(Debug, Copy, Clone, Default)]
pub struct Keyboard {
    pub strife_left:bool,
    pub strife_right:bool,
    pub move_forwad:bool,
    pub move_backward:bool
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Input {
    pub keyboard:Keyboard
}

pub struct Context<'a, W:GameWorld> {
    pub world:&'a mut World<W>,
    pub event:&'a Event<W::Event>,
    pub input:Input
}