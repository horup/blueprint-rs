use ggez::{event::KeyCode, input::{keyboard, mouse}, timer};
use glam::Vec2;

use crate::{context::Context, event::Event, input::{Input, Keyboard, Mouse}, systems::movement, world::GameWorld};
use super::Engine;


impl<W:GameWorld> Engine<W>  {

    
    fn get_input(&self, ctx:&ggez::Context) -> Input {
        let mouse_pos = mouse::position(ctx);
        let mut mouse_pos = Vec2::new((mouse_pos.x - self.config.width/2.0) / self.camera.zoom, (mouse_pos.y  - self.config.height/2.0) / self.camera.zoom);
        mouse_pos.x -= self.camera.pos.x;
        mouse_pos.y -= self.camera.pos.y;
        
        Input {
            keyboard: Keyboard {
                up:keyboard::is_key_pressed(ctx, KeyCode::W),
                down:keyboard::is_key_pressed(ctx, KeyCode::S),
                left:keyboard::is_key_pressed(ctx, KeyCode::A),
                right:keyboard::is_key_pressed(ctx, KeyCode::D)
            },
            mouse: Mouse {
                pos:mouse_pos,
                primary:false
            }
        }
    }

    fn push_event(&mut self, event:Event<W::Event>, ctx:&mut ggez::Context) {
        let input = self.get_input(&ctx);
        let engine_systems = [movement::movement];
        for system in engine_systems.iter() {
            let mut context = Context {
                event:&event,
                world:&mut self.world,
                input
            };

            system(&mut context);
        }

        for system in self.systems.clone() {
            let mut context = Context {
                event:&event,
                world:&mut self.world,
                input
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