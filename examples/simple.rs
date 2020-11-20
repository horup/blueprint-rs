use blueprint::{context::Context, engine::Engine, math::Rect2, art::Art, world::GameWorld};
use rand::random;

struct SimpleWorld {
    pub timer:f32
}

impl GameWorld for SimpleWorld {
    type Sprite = SimpleSprite;
    type Event = SimpleEvent;
    type Art = SimpleArt;
    type Texture = SimpleTexture;
}

#[derive(Debug, Copy, Clone, Default)]
struct SimpleSprite {
}

#[derive(Debug, Copy, Clone)]
enum SimpleEvent {
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
            timer:0.0
        }
    }
}

fn tick(ctx:&mut Context<SimpleWorld>) 
{
    match ctx.event {
        blueprint::event::Event::Tick(delta) => {
            let timer = &mut ctx.world.ext.timer;

            *timer += delta;

            if *timer > 0.2 {
                
                *timer = 0.0;
                            
                let mut s = ctx.world.new_sprite(SimpleArt::Zombie);
                let dy = 8.0;
                let dx = 16.0;
                s.pos.x = random::<f32>() * dx - dx / 2.0;
                s.frame = s.pos.x % 10.0;
                s.pos.y = -dy;


            }
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


    engine.systems.push(tick);
    Engine::run(engine);
}