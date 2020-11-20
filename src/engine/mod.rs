mod draw;
mod update;
use std::collections::HashMap;
use ggez::{Context, ContextBuilder, event::{self, EventHandler, EventsLoop}, graphics::{self, FilterMode}};
use ggez::graphics::{GlBackendSpec, ImageGeneric};

use crate::{camera::Camera, collection::Collection, config::Config, math::Rect2, spritetype::SpriteType, system::System, world::GameWorld, world::World};

pub struct Engine<W:GameWorld> {
    pub world:World<W>,
    pub systems:Vec<System<W>>,
    //pub sprite_types:HashMap<u32, SpriteType>,
    pub sprite_types:Collection<W::SpriteTypes, SpriteType>,
    pub config:Config,
    pub camera:Camera,
    textures:HashMap<u32, ImageGeneric<GlBackendSpec>>,
    ctx:ggez::Context,
    event_loop:EventsLoop
}

// TODO implement keyboard support
// TODO implement mouse support
// TODO implement cursor support
// TODO implement player controlling support

impl<W:GameWorld> Engine<W> {
    pub fn new() -> Self {
        let (mut ctx, mut event_loop) = ContextBuilder::new("game_id", "author")
        .build().expect("could not create context");

        let mut engine = Self {
            world:World::default(),
            systems:Vec::new(),
            config:Config::default(),
            textures:HashMap::new(),
            ctx:ctx,
            event_loop:event_loop,
            sprite_types:Collection::default(),
            camera:Camera::default()
        };

        graphics::set_default_filter(&mut engine.ctx, graphics::FilterMode::Nearest);
        let tex = include_bytes!("../resources/engine_spritesheet.png");
        engine.load_texture(tex, 0 as u32);

        let r = graphics::screen_coordinates(&engine.ctx);
        engine.config.width = r.w;
        engine.config.height = r.h;


        engine
    }

    pub fn load_texture<T:Into<u32>>(&mut self, bytes:&[u8], index:T) {
        let tex = image::load_from_memory(bytes).unwrap();
        let tex = tex.to_rgba();
        let mut tex = graphics::Image::from_rgba8(&mut self.ctx, tex.width() as u16, tex.height() as u16, &tex).unwrap();
        self.textures.insert(index.into(), tex);
        println!("load");
    }

    pub fn run(mut engine:Self) {
        let ctx:*mut Context = &mut engine.ctx;
        let event_loop:*mut EventsLoop = &mut engine.event_loop;

        unsafe {
            // unsafe is needed since engine owns the value.
            match event::run(&mut *ctx, &mut *event_loop, &mut engine) {
                Ok(_) => println!("Exited cleanly."),
                Err(e) => println!("Error occured: {}", e)
            }
        }
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