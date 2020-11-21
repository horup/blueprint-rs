use ggez::{timer};

use crate::{context::Context, event::Event, world::GameWorld};

use super::Engine;

fn movement<T:GameWorld>(ctx:&mut Context<T>)  {
    match ctx.event {
        Event::Tick(delta) => {
            for sprite in ctx.world.sprites_iter_mut() {
                sprite.pos += sprite.vel * *delta;
            }
        },
        _ => {}
    }
}

impl<W:GameWorld> Engine<W>  {
    pub(super) fn ggez_update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        while timer::check_update_time(ctx, self.config.tick_rate_ps) {
            let delta = 1.0 / self.config.tick_rate_ps as f32;  
            let event = Event::Tick(delta);

            let engine_systems = [movement];
            for system in engine_systems.iter() {
                let mut context = Context {
                    event:&event,
                    world:&mut self.world
                };
    
                system(&mut context);
            }

            for system in self.systems.clone() {
                let mut context = Context {
                    event:&event,
                    world:&mut self.world
                };
    
                system(&mut context);
            }
        }
       
        Result::Ok(())
    }
}