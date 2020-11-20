mod draw;
mod update;
use std::collections::HashMap;
use ggez::{ContextBuilder, event::{self, EventHandler}, graphics::{self, FilterMode}};
use ggez::graphics::{GlBackendSpec, ImageGeneric};

use crate::{camera::Camera, config::Config, math::Rect2, spritetype::SpriteType, system::System, world::GameWorld, world::World};
pub struct Engine<W:GameWorld> {
    pub world:World<W>,
    pub systems:Vec<System<W>>,
    pub sprite_types:HashMap<u32, SpriteType>,
    pub config:Config,
    pub camera:Camera,
    textures:HashMap<u32, ImageGeneric<GlBackendSpec>>,
    ctx:*mut ggez::Context,
}

// TODO implement keyboard support
// TODO implement mouse support
// TODO implement cursor support
// TODO implement player controlling support

impl<W:GameWorld> Engine<W> {
    pub fn new() -> Self {
        Self {
            world:World::default(),
            systems:Vec::new(),
            config:Config::default(),
            textures:HashMap::new(),
            ctx:std::ptr::null_mut(),
            sprite_types:HashMap::new(),
            camera:Camera::default()
        }
    }

    fn init(&mut self, ctx: &mut ggez::Context) {
        self.ctx = ctx;
        graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
        let tex = include_bytes!("../resources/engine_spritesheet.png");
        let tex = image::load_from_memory(tex).unwrap();
        let tex = tex.to_rgba();
        let tex = graphics::Image::from_rgba8(ctx, tex.width() as u16, tex.height() as u16, &tex).unwrap();
        self.textures.insert(0, tex);

        let sprite_type = SpriteType {
            texture_id:0,
            frames:Vec::from([Rect2::new(0, 0, 16, 16), Rect2::new(16, 0, 16, 16)]),
            animation:crate::spritetype::Animation::LoopBackForth,
            frames_per_second:1.0
        };
        self.sprite_types.insert(0, sprite_type);
    }

    pub fn load_texture<T:Into<u32>>(&mut self, bytes:&[u8], index:T) {
        if !self.ctx.is_null() {
            unsafe  {
                let tex = image::load_from_memory(bytes).unwrap();
                let tex = tex.to_rgba();
                let mut tex = graphics::Image::from_rgba8(&mut *self.ctx, tex.width() as u16, tex.height() as u16, &tex).unwrap();
                tex.set_filter(FilterMode::Nearest);
                self.textures.insert(index.into(), tex);
            }
        }
    }

    pub fn run(engine:Self) {
        let mut engine = engine;
        let (mut ctx, mut event_loop) = ContextBuilder::new("game_id", "author")
        .build().expect("could not create context");
        engine.init(&mut ctx);
        let r = graphics::screen_coordinates(&ctx);
        engine.config.width = r.w;
        engine.config.height = r.h;

        match event::run(&mut ctx, &mut event_loop, &mut engine) {
            Ok(_) => println!("Exited cleanly."),
            Err(e) => println!("Error occured: {}", e)
        }

        engine.ctx = std::ptr::null_mut();
    }

}

impl<W:GameWorld>  EventHandler for Engine<W>  {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.ggez_update(ctx)
    }


    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.ggez_draw(ctx)
    }
}