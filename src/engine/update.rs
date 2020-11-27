use ggez::{event::KeyCode, input::{keyboard, mouse}, timer};
use glam::{Vec3};

use crate::{context::Context, event::Event, input::{Input, Keyboard, Mouse}, systems::movement, world::GameWorld, systems::locomotion};
use super::Engine;


impl<W:GameWorld> Engine<W>  {
    fn get_input(&self, ctx:&ggez::Context) -> Input {
        let mouse_pos = mouse::position(ctx);
        /*let mut mouse_pos = Vec3::new((mouse_pos.x - self.config.width/2.0) / self.camera.zoom, (mouse_pos.y  - self.config.height/2.0) / self.camera.zoom, 0.0);
        mouse_pos.x -= self.camera.pos.x;
        mouse_pos.y -= self.camera.pos.y;*/

        let zoom_w = self.config.width / self.camera.zoom;
        let zoom_h = self.config.height / self.camera.zoom;
        let mut mouse_pos = Vec3::new(mouse_pos.x / self.config.width * zoom_w, mouse_pos.y / self.config.height * zoom_h, 0.0);
        mouse_pos.x += -self.camera.pos.x - zoom_w/2.0;
        mouse_pos.y += -self.camera.pos.y - zoom_h/2.0;
        Input {
            keyboard: Keyboard {
                up:keyboard::is_key_pressed(ctx, KeyCode::W),
                down:keyboard::is_key_pressed(ctx, KeyCode::S),
                left:keyboard::is_key_pressed(ctx, KeyCode::A),
                right:keyboard::is_key_pressed(ctx, KeyCode::D)
            },
            mouse: Mouse {
                pos:mouse_pos,
                primary:mouse::button_pressed(ctx, ggez::event::MouseButton::Left)
            }
        }
    }

    fn push_event(&mut self, event:Event<W::Event>, ctx:&mut ggez::Context) {
        let engine_systems = [
            locomotion::locomotion, 
            movement::movement];
        let mut new_events = Vec::new();
        let mut push_event = |e| {
            new_events.push(e);
        };
        for system in engine_systems.iter() {
            let mut context = Context {
                event:&event,
                world:&mut self.world,
                input:self.input,
                push_event:&mut push_event
            };

            system(&mut context);
        }

        for system in self.systems.clone() {
            let mut context = Context {
                event:&event,
                world:&mut self.world,
                input:self.input,
                push_event:&mut push_event
            };

            system(&mut context);
        }

        for e in new_events {
            self.push_event(e, ctx);
        }
    }

    pub(super) fn ggez_update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.input = self.get_input(&ctx);
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