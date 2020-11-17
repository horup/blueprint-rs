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
    let mut s = engine.world_mut().new_sprite();
    
    engine.config.window_title = "Simple Example".into();
    engine.systems_mut().push(tick);
    Engine::run(engine);
}