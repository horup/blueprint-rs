use ggez::{ContextBuilder, event::{self, EventHandler}};

use crate::{config::Config, system::System, world::World};



pub struct Engine<W:Default,S:Default,E> {
    world:World<W,S>,
    systems:Vec<System<W,S,E>>,
    pub config:Config
}

impl<W:Default, S:Default, E> Engine<W, S, E> {
    pub fn new() -> Self {
        Self {
            world:World::default(),
            systems:Vec::new(),
            config:Config::default()
        }
    }
    
    pub fn systems_mut(&mut self) -> &mut Vec<System<W,S,E>> {
        &mut self.systems
    }
    
    pub fn systems(&self) -> &Vec<System<W,S,E>> {
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

impl<W:Default, S:Default, E>  EventHandler for Engine<W, S, E>  {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {

        Result::Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Result::Ok(())
    }
}