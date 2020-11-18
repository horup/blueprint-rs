use crate::{context::Context};

pub type System<W> = fn(&mut Context<W>);