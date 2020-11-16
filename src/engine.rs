use ggez::{ContextBuilder, event::{self, EventHandler}, graphics, timer};

use crate::{config::Config, context::Context, event::Event, system::System, world::GameWorld, world::World};

pub struct Engine<W:GameWorld> {
    world:World<W>,
    systems:Vec<System<W>>,
    pub config:Config
}

impl<W:GameWorld> Engine<W> {
    pub fn new() -> Self {
        Self {
            world:World::default(),
            systems:Vec::new(),
            config:Config::default()
        }
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

        match event::run(&mut ctx, &mut event_loop, &mut engine) {
            Ok(_) => println!("Exited cleanly."),
            Err(e) => println!("Error occured: {}", e)
        }
    }
}

impl<W:GameWorld>  EventHandler for Engine<W>  {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let event = Event::Tick(timer::average_delta(ctx).as_secs_f32());

        for system in self.systems().clone() {
            let mut context = Context {
                event:&event,
                world:&mut self.world
            };

            system(&mut context);
        }
        Result::Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Result::Ok(())
    }
}