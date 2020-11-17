use std::collections::HashMap;

use ggez::{ContextBuilder, event::{self, EventHandler}, graphics::Color, graphics::{self, DrawParam, Image}, mint::{Vector2, Point2}, mint::{self}, timer};
use ggez::graphics::{GlBackendSpec, ImageGeneric, Rect};

use crate::{config::Config, context::Context, event::Event, system::System, world::GameWorld, world::World};
pub struct Engine<W:GameWorld> {
    world:World<W>,
    systems:Vec<System<W>>,
    textures:HashMap<u16, ImageGeneric<GlBackendSpec>>,
    ctx:*mut ggez::Context,
    pub config:Config
}

impl<W:GameWorld> Engine<W> {
    // TODO: implement texture loading, spritesheets, etc
    pub fn new() -> Self {
        Self {
            world:World::default(),
            systems:Vec::new(),
            config:Config::default(),
            textures:HashMap::new(),
            ctx:std::ptr::null_mut()
        }
    }

    fn init(&mut self, ctx: &mut ggez::Context) {
        self.ctx = ctx;
        let tex = include_bytes!("./resources/engine_spritesheet.png");
        let tex = image::load_from_memory(tex).unwrap();
        let tex = tex.to_rgba();
        let tex = graphics::Image::from_rgba8(ctx, tex.width() as u16, tex.height() as u16, &tex).unwrap();
        self.textures.insert(0, tex);
        // TODO: finish implmeneting of sprite sheet saving
    }

    pub fn load_texture<T:Into<u16>>(&mut self, bytes:&[u8], index:T) {
        if !self.ctx.is_null() {
            unsafe  {
                let tex = image::load_from_memory(bytes).unwrap();
                let tex = tex.to_rgba();
                let tex = graphics::Image::from_rgba8(&mut *self.ctx, tex.width() as u16, tex.height() as u16, &tex).unwrap();
                self.textures.insert(index.into(), tex);
            }
        }
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

        graphics::clear(ctx, Color::from_rgb(0, 0, 0) );
        
        for sprite in self.world.sprites_iter() {
            if let Some(img) = self.textures.get(&0) {
                graphics::draw(ctx, img, DrawParam {
                    ..DrawParam::default()
                })?;
            }
            
        }

        self.draw_debug(ctx)?;

        graphics::present(ctx)?;
        Result::Ok(())
    }
}