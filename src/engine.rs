use std::collections::HashMap;

use ggez::{ContextBuilder, event::{self, EventHandler}, graphics::Color, graphics::{self, DrawParam, Image}, mint::{Vector2, Point2}, mint::{self}, timer};
use ggez::graphics::{GlBackendSpec, ImageGeneric, Rect};
use glam::Vec2;

use crate::{config::Config, context::Context, event::Event, math::Rect2, sprite::Sprite, spritetype::SpriteType, system::System, world::GameWorld, world::World};
pub struct Engine<W:GameWorld> {
    world:World<W>,
    systems:Vec<System<W>>,
    textures:HashMap<u32, ImageGeneric<GlBackendSpec>>,
    ctx:*mut ggez::Context,
    pub sprite_types:HashMap<u32, SpriteType>,
    pub config:Config
}

// TODO: move stuff to own types as to avoid borrow checking

impl<W:GameWorld> Engine<W> {
    pub fn new() -> Self {
        Self {
            world:World::default(),
            systems:Vec::new(),
            config:Config::default(),
            textures:HashMap::new(),
            ctx:std::ptr::null_mut(),
            sprite_types:HashMap::new()
        }
    }

    fn init(&mut self, ctx: &mut ggez::Context) {
        self.ctx = ctx;
        let tex = include_bytes!("./resources/engine_spritesheet.png");
        let tex = image::load_from_memory(tex).unwrap();
        let tex = tex.to_rgba();
        let tex = graphics::Image::from_rgba8(ctx, tex.width() as u16, tex.height() as u16, &tex).unwrap();
        self.textures.insert(0, tex);

        let sprite_type = SpriteType {
            texture_id:0,
            frames:Vec::from([Rect2::new(0, 0, 16, 16), Rect2::new(16, 0, 16, 16)]),
            animation:crate::spritetype::Animation::LoopBackForth,
            animation_speed_ps:1.0
        };
        // TODO: finish implmeneting of sprite sheet saving
        self.load_sprite_type(sprite_type, 0);
    }

    pub fn load_texture<T:Into<u32>>(&mut self, bytes:&[u8], index:T) {
        if !self.ctx.is_null() {
            unsafe  {
                let tex = image::load_from_memory(bytes).unwrap();
                let tex = tex.to_rgba();
                let tex = graphics::Image::from_rgba8(&mut *self.ctx, tex.width() as u16, tex.height() as u16, &tex).unwrap();
                self.textures.insert(index.into(), tex);
            }
        }
    }

    pub fn load_sprite_type<T:Into<SpriteType>>(&mut self, sprite_type:T, index:u32) {
        self.sprite_types.insert(index, sprite_type.into());
    }

    pub fn get_sprite_type(&self, index:u32) -> Option<&SpriteType> {
        self.sprite_types.get(&index)
    }

    pub fn get_sprite_type_mut(&mut self, index:u32) -> Option<&mut SpriteType> {
        self.sprite_types.get_mut(&index)
    }

    pub fn world(&self) -> &World<W> {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World<W> {
        &mut self.world
    }
    
    pub fn systems_mut(&mut self) -> &mut Vec<System<W>> {
        &mut self.systems
    }
    
    pub fn systems(&self) -> &Vec<System<W>> {
        &self.systems
    }

    pub fn run(engine:Self) {
        let mut engine = engine;
        let (mut ctx, mut event_loop) = ContextBuilder::new("game_id", "author")
        .build().expect("could not create context");
        engine.init(&mut ctx);

        match event::run(&mut ctx, &mut event_loop, &mut engine) {
            Ok(_) => println!("Exited cleanly."),
            Err(e) => println!("Error occured: {}", e)
        }

        engine.ctx = std::ptr::null_mut();
    }

    fn draw_debug(&mut self, ctx:&mut ggez::Context) -> ggez::GameResult {
        let alpha = timer::remaining_update_time(ctx).as_millis() as f32 / (1000.0 / self.config.tick_rate_ps as f32);
        let text = graphics::Text::new(format!("FPS: {}", timer::fps(ctx) as i32));
        graphics::draw(ctx, &text, DrawParam {
            dest:[0.0, 0.0].into(),
            ..Default::default()
        })?;

        Result::Ok(())
    }


}

impl<W:GameWorld>  EventHandler for Engine<W>  {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        while timer::check_update_time(ctx, self.config.tick_rate_ps) {
            let delta = 1.0 / self.config.tick_rate_ps as f32;  
            let event = Event::Tick(delta);
            for system in self.systems().clone() {
                let mut context = Context {
                    event:&event,
                    world:&mut self.world
                };
    
                system(&mut context);
            }
        }
       
        Result::Ok(())
    }


    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::set_window_title(ctx, &self.config.window_title);
        // TODO: Implement draw
        // TODO: Implement interpolation
        // BUG: Alpha sometimes returns a big number?
        // TODO: Finish animation of sprite
        graphics::clear(ctx, Color::from_rgb(0, 0, 0) );

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
                        sprite.frame += dt;
                        if sprite.frame > sprite_type.frames.len() as f32 {
                            sprite.frame = 0.0;
                        }
                    }
                    crate::spritetype::Animation::LoopBackForth => {
                        if sprite.animation_reverse { sprite.frame -= dt} else { sprite.frame += dt};
                        if sprite.frame > sprite_type.frames.len() as f32 {
                            sprite.frame = sprite_type.frames.len() as f32 - 1.0;
                            sprite.animation_reverse = true;
                        }
                        else if sprite.frame <= 0.0 {
                            sprite.frame = 0.0;
                            sprite.animation_reverse = false;
                        }
                    }
                    crate::spritetype::Animation::ForwardStop => {
                        sprite.frame += dt;
                        if sprite.frame > sprite_type.frames.len() as f32 {
                            sprite.frame = sprite_type.frames.len() as f32 - 1.0;
                        }
                    }
                }
            }
        }
        
        for sprite in self.world.sprites_iter() {
            if let Some(sprite_type) = self.get_sprite_type(sprite.sprite_type_id) {
                if sprite_type.frames.len() > 0 {
                    if let Some(img) = self.textures.get(&sprite_type.texture_id) {
                        let frame = sprite.frame as usize % sprite_type.frames.len();
                        if let Some(frame) = sprite_type.frames.get(frame) {
                            let mut src = Rect::new(0.0, 0.0, img.width() as f32, img.height() as f32);
                            src.x = frame.x as f32 / src.w;
                            src.y = frame.y as f32 / src.h;
                            src.w = frame.w as f32 / src.w;
                            src.h = frame.h as f32 / src.h;
                            let dest:Point2<f32> = Vec2::new(sprite.pos.x, sprite.pos.y).into();
                            graphics::draw(ctx, img, DrawParam {
                                dest,
                                src,
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