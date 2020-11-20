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



    pub(super) fn ggez_draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::set_window_title(ctx, &self.config.window_title);
        // TODO: Implement interpolation
        // BUG: Alpha sometimes returns a big number?
        graphics::clear(ctx, Color::from_rgb(0, 0, 0) );

        let config = &self.config;
        let camera = &self.camera;
        let mut r = Rect::new(camera.pos.x, camera.pos.x, config.width / camera.zoom, config.height / camera.zoom);
        r.x -= r.w / 2.0;
        r.y -= r.h / 2.0;
        graphics::set_screen_coordinates(ctx, r)?;

        /*let sprite_types = self.sprite_types.clone();
        for sprite in self.world.sprites_iter_mut() {
            sprite.frame += timer::average_delta(ctx).as_secs_f32();
        }*/
        let dt = timer::average_delta(ctx).as_secs_f32();
        for sprite in self.world.sprites_iter_mut() {
            if let Some(sprite_type) = self.sprite_types.get(&sprite.sprite_type_id) {
                match sprite_type.animation 
                {
                    crate::spritetype::Animation::None => {}
                    crate::spritetype::Animation::Loop => {
                        sprite.frame += dt * sprite_type.frames_per_second;
                        println!("{}", sprite_type.frames_per_second);
                        if sprite.frame > sprite_type.frames.len() as f32 {
                            sprite.frame = 0.0;
                        }
                    }
                    crate::spritetype::Animation::LoopBackForth => {
                        let dt = dt * sprite_type.frames_per_second;
                        if sprite.animation_reverse { sprite.frame -= dt} else { sprite.frame += dt};
                        if sprite.frame > sprite_type.frames.len() as f32 {
                            sprite.frame = sprite_type.frames.len() as f32 - 1.0;
                            sprite.animation_reverse = true;
                        }
                        else if sprite.frame <= 0.0 {
                            sprite.frame = 0.99;
                            sprite.animation_reverse = false;
                        }
                    }
                    crate::spritetype::Animation::ForwardStop => {
                        sprite.frame += dt * sprite_type.frames_per_second;
                        if sprite.frame > sprite_type.frames.len() as f32 {
                            sprite.frame = sprite_type.frames.len() as f32 - 1.0;
                        }
                    }
                }
            }
        }
        
        for sprite in self.world.sprites_iter() {
            if let Some(sprite_type) = self.sprite_types.get(&sprite.sprite_type_id) {
                if sprite_type.frames.len() > 0 {
                    if let Some(img) = self.textures.get(&sprite_type.texture_id) {
                        let frame = sprite.frame as usize % sprite_type.frames.len();
                        if let Some(frame) = sprite_type.frames.get(frame) {
                            let mut src = Rect::new(0.0, 0.0, img.width() as f32, img.height() as f32);
                            src.x = frame.x as f32 / src.w;
                            src.y = frame.y as f32 / src.h;
                            src.w = frame.w as f32 / src.w;
                            src.h = frame.h as f32 / src.h;
                            
                            let mut scale:Vector2<f32> = Vec2::new(1.0 / img.width() as f32 * frame.w, 1.0 / img.height() as f32 * frame.h).into();
                            scale.x *= sprite_type.width * sprite.scale.y;
                            scale.y *= sprite_type.height * sprite.scale.x;
                            let dest:Point2<f32> = Vec2::new(sprite.pos.x, sprite.pos.y).into();
                            graphics::draw(ctx, img, DrawParam {
                                dest,
                                src,
                                scale,
                                ..DrawParam::default()
                            })?;

                        }
                    }
                }
            }
        }

        self.draw_debug(ctx)?;
        graphics::present(ctx)?;
        Result::Ok(())
    }
}