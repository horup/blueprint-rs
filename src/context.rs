use crate::{event::Event, input::Input, world::{GameWorld, World}};

pub struct Context<'a, W:GameWorld> {
    pub world:&'a mut World<W>,
    pub event:&'a Event<W::Event>,
    pub input:Input,
    pub push_event:&'a mut dyn FnMut(Event<W::Event>)
}