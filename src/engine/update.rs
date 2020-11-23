use ggez::{event::KeyCode, input::{keyboard}, timer};

use crate::{context::Context, context::Input, event::Event, context::Keyboard, systems::movement, world::GameWorld};
use super::Engine;

fn input(ctx:&ggez::Context) -> Input {
    Input {
        keyboard: Keyboard {
            up:keyboard::is_key_pressed(ctx, KeyCode::W),
            down:keyboard::is_key_pressed(ctx, KeyCode::S),
            left:keyboard::is_key_pressed(ctx, KeyCode::A),
            right:keyboard::is_key_pressed(ctx, KeyCode::D)
        }
    }
}

impl<W:GameWorld> Engine<W>  {
    fn push_event(&mut self, event:Event<W::Event>, ctx:&mut ggez::Context) {
        let engine_systems = [movement::movement];
        for system in engine_systems.iter() {
            let mut context = Context {
                event:&event,
                world:&mut self.world,
                input:input(&ctx)
            };

            system(&mut context);
        }

        for system in self.systems.clone() {
            let mut context = Context {
                event:&event,
                world:&mut self.world,
                input:input(&ctx)
            };

            system(&mut context);
        }
    }

    pub(super) fn ggez_update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        while timer::check_update_time(ctx, self.config.tick_rate_ps) {
            let prev_snapshot = self.world.clone();
            if self.prev_snapshots.len() > 20 {
                self.prev_snapshots.pop_back();
            }

            self.prev_snapshots.push_front(prev_snapshot);
            let delta = 1.0 / self.config.tick_rate_ps as f32;  
            let event = Event::Update(delta);
            self.world.timestamp += delta;
            self.push_event(event, ctx);
        }

        let delta = timer::average_delta(ctx).as_secs_f32();
        let event = Event::Draw(delta);
        self.push_event(event, ctx);
        Result::Ok(())
    }
}