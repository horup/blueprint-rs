use blueprint::{context::Context, engine::Engine, math::Rect2, spritetype::SpriteType, world::GameWorld};

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
        },
        _ => {}
    }
}

fn main() {
    let mut engine:Engine<SimpleWorld> = Engine::new();
    engine.load_texture(include_bytes!("spritesheet.png"), 1 as u16);
    engine.sprite_types.insert(1, SpriteType {
        animation : blueprint::spritetype::Animation::LoopBackForth,
        frames:[Rect2::new(0.0, 0.0, 16.0, 16.0), Rect2::new(0.0, 16.0, 16.0, 16.0)].into(),
        frames_per_second:2.0,
        height:16.0,
        width:16.0,
        texture_id:1
    });

    engine.config.window_title = "Simple Example".into();

    let mut s = engine.world.new_sprite();
    s.sprite_type_id = 1;
    s.pos.x = 0.0;
    s.pos.y = 0.0;

    engine.systems.push(tick);
    Engine::run(engine);
}