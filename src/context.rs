use crate::{event::Event, world::{GameWorld, World}};

pub struct Context<'a, W:GameWorld> {
    pub world:&'a mut World<W>,
    pub event:&'a Event<W::Event>
}