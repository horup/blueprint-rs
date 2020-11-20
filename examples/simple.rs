use blueprint::{context::Context, engine::Engine, math::Rect2, spritetype::SpriteType, world::GameWorld};

struct SimpleWorld {
    pub timer:f32
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum SimpleSprites {
    Player,
    Zombie
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
    type SpriteTypes = SimpleSprites;
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
    engine.camera.zoom = 32.0;
    engine.load_texture(include_bytes!("spritesheet.png"), 1 as u16);
    engine.sprite_types.insert(SimpleSprites::Player, SpriteType {
        animation : blueprint::spritetype::Animation::LoopBackForth,
        frames:[Rect2::new(0.0, 0.0, 16.0, 16.0), Rect2::new(0.0, 16.0, 16.0, 16.0)].into(),
        frames_per_second:2.0,
        height:1.0,
        width:1.0,
        texture_id:1
    });
    engine.sprite_types.insert(SimpleSprites::Zombie,SpriteType {
        animation : blueprint::spritetype::Animation::LoopBackForth,
        frames:[Rect2::new(16.0, 0.0, 16.0, 16.0), Rect2::new(16.0, 16.0, 16.0, 16.0)].into(),
        frames_per_second:2.0,
        height:1.0,
        width:1.0,
        texture_id:1
    });
  
    engine.config.window_title = "Simple Example".into();

    let mut s = engine.world.new_sprite(SimpleSprites::Player);
    s.pos.x = 0.0;
    s.pos.y = 0.0;

    let mut spawn_zombie = |x,y| {
        let mut s = engine.world.new_sprite(SimpleSprites::Zombie);
        s.frame = x % 10.0;
        s.pos.x = x;
        s.pos.y = y;
    };

    spawn_zombie(10.0, 0.0);

    engine.systems.push(tick);
    Engine::run(engine);
}