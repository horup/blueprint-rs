use crate::{context::Context, world::GameWorld};

pub type System<W:GameWorld> = fn(&mut Context<W>);