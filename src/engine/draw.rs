use ggez::{self, graphics::{self, Color, DrawParam, Rect}, mint::Point2, mint::Vector2, timer};
use glam::Vec2;
use crate::world::GameWorld;
use super::Engine;

impl<W:GameWorld> Engine<W> {
    fn draw_debug(&mut self, ctx:&mut ggez::Context) -> ggez::GameResult {
        graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, self.config.width, self.config.height))?;
        let text = graphics::Text::new(format!("FPS: {}", timer::fps(ctx) as i32));
        graphics::draw(ctx, &text, DrawParam {
            dest:[0.0, 0.0].into(),
            ..Default::default()
        })?;

        Result::Ok(())
    }


    fn draw_sprites(&mut self, ctx:&mut ggez::Context, alpha:f32, dt:f32) -> ggez::GameResult {
        if let Some(prev_snapshot) = self.prev_snapshots.front() {
            for current_sprite in self.world.sprites_iter_mut() {
                if let Some(sprite_type) = self.art.get(&current_sprite.art) {
                    match sprite_type.animation 
                    {
                        crate::art::Animation::None => {}
                        crate::art::Animation::Loop => {
                            current_sprite.frame += dt * sprite_type.frames_per_second;
                            println!("{}", sprite_type.frames_per_second);
                            if current_sprite.frame > sprite_type.frames.len() as f32 {
                                current_sprite.frame = 0.0;
                            }
                        }
                        crate::art::Animation::LoopBackForth => {
                            let dt = dt * sprite_type.frames_per_second;
                            if current_sprite.animation_reverse { current_sprite.frame -= dt} else { current_sprite.frame += dt};
                            if current_sprite.frame > sprite_type.frames.len() as f32 {
                                current_sprite.frame = sprite_type.frames.len() as f32 - 1.0;
                                current_sprite.animation_reverse = true;
                            }
                            else if current_sprite.frame <= 0.0 {
                                current_sprite.frame = 0.99;
                                current_sprite.animation_reverse = false;
                            }
                        }
                        crate::art::Animation::ForwardStop => {
                            current_sprite.frame += dt * sprite_type.frames_per_second;
                            if current_sprite.frame > sprite_type.frames.len() as f32 {
                                current_sprite.frame = sprite_type.frames.len() as f32 - 1.0;
                            }
                        }
                    }
                }
            }
            
            for current_sprite in self.world.sprites_iter() {
                let mut f = || {
                    let sprite_type = self.art.get(&current_sprite.art)?;
                    let img = self.textures.get(&sprite_type.texture_id)?;
                    let prev_sprite = prev_snapshot.get_sprite(current_sprite.id()).unwrap_or(current_sprite);
                    let frame = current_sprite.frame as usize % sprite_type.frames.len();
                    let frame = sprite_type.frames.get(frame)?;

                    let mut pos = (current_sprite.pos - prev_sprite.pos) * alpha + prev_sprite.pos;

                    let mut src = Rect::new(0.0, 0.0, img.width() as f32, img.height() as f32);
                    src.x = frame.x as f32 / src.w;
                    src.y = frame.y as f32 / src.h;
                    src.w = frame.w as f32 / src.w;
                    src.h = frame.h as f32 / src.h;
                    
                    let mut scale:Vector2<f32> = Vec2::new(1.0 / img.width() as f32 * frame.w, 1.0 / img.height() as f32 * frame.h).into();
                    scale.x *= sprite_type.width * current_sprite.scale.y;
                    scale.y *= sprite_type.height * current_sprite.scale.x;
                    let dest:Point2<f32> = Vec2::new(pos.x, pos.y).into();
                    let _ = graphics::draw(ctx, img, DrawParam {
                        dest,
                        src,
                        scale,
                        ..DrawParam::default()
                    });

                    Some(())
                };
                f();
            }
        }

        Ok(())
    }

    pub(super) fn ggez_draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let alpha = timer::remaining_update_time(ctx).as_secs_f32() as f32 * self.config.tick_rate_ps as f32;
        graphics::set_window_title(ctx, &self.config.window_title);
        graphics::clear(ctx, Color::from_rgb(0, 0, 0) );

        let config = &self.config;
        let camera = &self.camera;
        let mut r = Rect::new(camera.pos.x, camera.pos.x, config.width / camera.zoom, config.height / camera.zoom);
        r.x -= r.w / 2.0;
        r.y -= r.h / 2.0;
        graphics::set_screen_coordinates(ctx, r)?;

        let dt = timer::average_delta(ctx).as_secs_f32();

      
        self.draw_sprites(ctx, alpha, dt)?;
        self.draw_debug(ctx)?;
        graphics::present(ctx)?;
        Result::Ok(())
    }
}