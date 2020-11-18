use blueprint::{context::Context, engine::Engine, world::GameWorld};

struct SimpleWorld {
    pub timer:f32
}

impl Default for SimpleWorld {
    fn default() -> Self {
        Self {
            timer:100.0
        }
    }
}

impl GameWorld for SimpleWorld {
    type Sprite = ();
    type Event = ();
}

fn tick(ctx:&mut Context<SimpleWorld>) 
{
    
    match ctx.event {
        blueprint::event::Event::Tick(delta) => {
            println!("{}", delta);
            ctx.world.game_mut().timer += delta;
            println!("{}", ctx.world.game().timer);
        },
        _ => {}
    }
}

fn main() {
    let mut engine:Engine<SimpleWorld> = Engine::new();
    engine.load_texture(include_bytes!("spritesheet.png"), 1 as u16);

    engine.config.window_title = "Simple Example".into();

    let mut s = engine.world_mut().new_sprite();
    s.pos.x = 10.0;
    s.pos.y = 20.0;

    engine.systems_mut().push(tick);
    Engine::run(engine);
}