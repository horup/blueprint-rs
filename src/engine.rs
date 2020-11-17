use ggez::{graphics::Color, ContextBuilder, event::{self, EventHandler}, graphics::{self, DrawParam}, mint::{Vector2, Point2}, mint::{self}, timer};
use glam::Vec2;

use crate::{config::Config, context::Context, event::Event, system::System, world::GameWorld, world::World};

pub struct Engine<W:GameWorld> {
    world:World<W>,
    systems:Vec<System<W>>,
    pub config:Config
}

impl<W:GameWorld> Engine<W> {
    // TODO: implement texture loading, spritesheets, etc
    pub fn new() -> Self {
        Self {
            world:World::default(),
            systems:Vec::new(),
            config:Config::default()
        }
    }

    fn init(&mut self, ctx: &mut ggez::Context) {
        let sprites = include_bytes!("./resources/engine_spritesheet.png");
        let sprites = image::load_from_memory(sprites).unwrap();
        let sprites = sprites.to_rgba();
        let sprites = graphics::Image::from_rgba8(ctx, sprites.width() as u16, sprites.height() as u16, &sprites).unwrap();
        // TODO: finish implmeneting of sprite sheet saving
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
        
        self.draw_debug(ctx)?;

        graphics::present(ctx)?;
        Result::Ok(())
    }
}