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
            ctx.world.game_mut().timer += delta;
            println!("{}", ctx.world.game().timer);
        },
        _ => {}
    }
}

fn main() {
    let mut engine:Engine<SimpleWorld> = Engine::new();
    engine.systems_mut().push(|ctx| {
        println!("hello from closue");
    });

    engine.systems_mut().push(tick);
    Engine::run(engine);
}