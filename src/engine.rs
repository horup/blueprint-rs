use ggez::{ContextBuilder, event::{self, EventHandler}};

use crate::world::World;

pub struct Engine<W:Default,S:Default> {
    current:World<W,S>
}

impl<W:Default, S:Default> Engine<W, S> {
   
    pub fn run(title:&str, initial:World<W,S>) {
        let (mut ctx, mut event_loop) = ContextBuilder::new("game_id", "author")
        .build().expect("could not create context");

        let mut engine = Self {
            current:initial
        };

        match event::run(&mut ctx, &mut event_loop, &mut engine) {
            Ok(_) => println!("Exited cleanly."),
            Err(e) => println!("Error occured: {}", e)
        }
    }
}

impl<W:Default, S:Default>  EventHandler for Engine<W, S>  {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        todo!()
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        todo!()
    }
}