use blueprint::{context::Context, engine::Engine, math::Rect2, art::Art, world::GameWorld};
use rand::random;

struct ZombieWorld {
    pub timer:f32
}

impl GameWorld for ZombieWorld {
    type Sprite = ZombieSprite;
    type Event = ZombieEvent;
    type Art = ZombieArt;
    type Texture = ZombieTexture;
}

#[derive(Debug, Copy, Clone, Default)]
struct ZombieSprite {
}

#[derive(Debug, Copy, Clone)]
enum ZombieEvent {
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum ZombieArt {
    Player,
    Zombie
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum ZombieTexture {
    Spritesheet
}

impl Default for ZombieWorld {
    fn default() -> Self {
        Self {
            timer:0.0
        }
    }
}

fn tick(ctx:&mut Context<ZombieWorld>) 
{
    match ctx.event {
        blueprint::event::Event::Tick(delta) => {
            let timer = &mut ctx.world.ext.timer;

            *timer += delta;

            if *timer > 0.2 {
                
                *timer = 0.0;
                            
                let mut s = ctx.world.new_sprite(ZombieArt::Zombie);
                let dy = 8.0;
                let dx = 16.0;
                s.pos.x = random::<f32>() * dx - dx / 2.0;
                s.frame = s.pos.x % 10.0;
                s.pos.y = -dy;
                s.vel.y = 1.0;


            }
        },
        _ => {}
    }
}

fn main() {
    let mut engine:Engine<ZombieWorld> = Engine::new();
    engine.camera.zoom = 32.0;
    engine.load_texture(include_bytes!("spritesheet.png"), ZombieTexture::Spritesheet);
    engine.art.insert(ZombieArt::Player, Art {
        animation : blueprint::art::Animation::LoopBackForth,
        frames:[Rect2::new(0.0, 0.0, 16.0, 16.0), Rect2::new(0.0, 16.0, 16.0, 16.0)].into(),
        frames_per_second:2.0,
        height:1.0,
        width:1.0,
        texture_id:ZombieTexture::Spritesheet
    });
    engine.art.insert(ZombieArt::Zombie,Art {
        animation : blueprint::art::Animation::LoopBackForth,
        frames:[Rect2::new(16.0, 0.0, 16.0, 16.0), Rect2::new(16.0, 16.0, 16.0, 16.0)].into(),
        frames_per_second:2.0,
        height:1.0,
        width:1.0,
        texture_id:ZombieTexture::Spritesheet
    });
  
    engine.config.window_title = "Zombie Example".into();

    let mut s = engine.world.new_sprite(ZombieArt::Player);
    s.pos.x = 0.0;
    s.pos.y = 0.0;


    engine.systems.push(tick);
    Engine::run(engine);
}