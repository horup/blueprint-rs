use blueprint::{context::Context, engine::Engine, math::Rect2, art::Art, world::GameWorld};

struct SimpleWorld {
    pub timer:f32
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum SimpleArt {
    Player,
    Zombie
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum SimpleTexture {
    Spritesheet
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
    type Art = SimpleArt;
    type Texture = SimpleTexture;
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
    engine.load_texture(include_bytes!("spritesheet.png"), SimpleTexture::Spritesheet);
    engine.art.insert(SimpleArt::Player, Art {
        animation : blueprint::art::Animation::LoopBackForth,
        frames:[Rect2::new(0.0, 0.0, 16.0, 16.0), Rect2::new(0.0, 16.0, 16.0, 16.0)].into(),
        frames_per_second:2.0,
        height:1.0,
        width:1.0,
        texture_id:SimpleTexture::Spritesheet
    });
    engine.art.insert(SimpleArt::Zombie,Art {
        animation : blueprint::art::Animation::LoopBackForth,
        frames:[Rect2::new(16.0, 0.0, 16.0, 16.0), Rect2::new(16.0, 16.0, 16.0, 16.0)].into(),
        frames_per_second:2.0,
        height:1.0,
        width:1.0,
        texture_id:SimpleTexture::Spritesheet
    });
  
    engine.config.window_title = "Simple Example".into();

    let mut s = engine.world.new_sprite(SimpleArt::Player);
    s.pos.x = 0.0;
    s.pos.y = 0.0;

    let mut spawn_zombie = |x,y| {
        let mut s = engine.world.new_sprite(SimpleArt::Zombie);
        s.frame = x % 10.0;
        s.pos.x = x;
        s.pos.y = y;
    };

    spawn_zombie(10.0, 0.0);

    engine.systems.push(tick);
    Engine::run(engine);
}