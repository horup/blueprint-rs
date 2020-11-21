use crate::{context::Context, event::Event, world::GameWorld};


pub fn movement<T:GameWorld>(ctx:&mut Context<T>)  {
    match ctx.event {
        Event::Tick(delta) => {
            for sprite in ctx.world.sprites_iter_mut() {
                sprite.pos += sprite.vel * *delta;
            }
        },
        _ => {}
    }
}