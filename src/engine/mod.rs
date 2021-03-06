mod draw;
mod update;
use std::collections::{HashMap, VecDeque};

use event::KeyCode;
use ggez::{Context, ContextBuilder, conf::{WindowMode, WindowSetup}, event::{self, EventHandler, EventsLoop}, graphics::{self}};
use ggez::graphics::{GlBackendSpec, ImageGeneric};

use crate::{art::Art, camera::Camera, collection::Collection, config::Config, system::System, world::GameWorld, input::Input, world::World};

pub struct Engine<W:GameWorld> {
    pub world:World<W>,
    pub prev_snapshots:VecDeque<World<W>>,
    pub systems:Vec<System<W>>,
    pub art:Collection<W::Art, Art<W>>,
    pub config:Config,
    pub camera:Camera,
    pub input:Input,
    pub keyboard_map:HashMap<KeyCode, bool>,
    textures:Collection<W::Texture, ImageGeneric<GlBackendSpec>>,
    ctx:ggez::Context,
    event_loop:EventsLoop
}

// TODO: improve system handling
impl<W:GameWorld> Engine<W> {
    pub fn new(title:String) -> Self {

        let mut config = Config::default();
        config.window_title = title.clone();
        let mut window_mode = WindowMode::default();
        let mut window_setup = WindowSetup::default();

        window_setup.title = title;
        window_setup.vsync = false;

        let (ctx, event_loop) = ContextBuilder::new("game_id", "author")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build().expect("could not create context");

        let mut engine = Self {
            world:World::default(),
            prev_snapshots:VecDeque::new(),
            systems:Vec::new(),
            config,
            textures:Collection::default(),
            ctx:ctx,
            event_loop:event_loop,
            keyboard_map:HashMap::new(),
            art:Collection::default(),
            camera:Camera::default(),
            input:Input::default()
        };

        graphics::set_default_filter(&mut engine.ctx, graphics::FilterMode::Nearest);
        let r = graphics::screen_coordinates(&engine.ctx);
        engine.config.width = r.w;
        engine.config.height = r.h;


        engine
    }

    pub fn load_texture(&mut self, bytes:&[u8], texture:W::Texture) {
        let tex = image::load_from_memory(bytes).unwrap();
        let tex = tex.to_rgba();
        let tex = graphics::Image::from_rgba8(&mut self.ctx, tex.width() as u16, tex.height() as u16, &tex).unwrap();
        self.textures.insert(texture, tex);
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