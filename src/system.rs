use crate::context::Context;

pub type System<W, S, E> = fn(&mut Context<W, S, E>);