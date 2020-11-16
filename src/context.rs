use crate::{event::Event, world::World};

pub struct Context<'a, W:Default, S:Default, E> {
    pub world:&'a mut World<W,S>,
    pub event:&'a Event<E>
}