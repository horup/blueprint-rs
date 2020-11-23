use blueprint::{art::Art, context::Context, engine::Engine, event::Event, math::Rect2, world::GameWorld};
use rand::random;

#[derive(Clone)]
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

fn update(ctx:&mut Context<ZombieWorld>) 
{
    match ctx.event {
        blueprint::event::Event::Update(delta) => {
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

// BUG: cap vel to speed
// TODO: 1) implement shooting
// TODO: move movement code to engine, since this can be reused
// TODO: implement zombie touch
// TODO: implement health
// TODO: implement AI
fn draw(ctx:&mut Context<ZombieWorld>) {
    match  ctx.event {
        Event::Draw(_delta) => {
            if let Some(player) = ctx.world.find_sprite_mut(|x| {x.art == ZombieArt::Player}) {
                let k = ctx.input.keyboard;
                let x = if k.left { -1.0 } else if k.right { 1.0 } else { 0.0 };
                let y = if k.up { -1.0 } else if k.down { 1.0 } else { 0.0 };
                let speed = 2.0;
                player.vel.x = x * speed;
                player.vel.y = y * speed;
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
        texture_id:ZombieTexture::Spritesheet,
    });
    engine.art.insert(ZombieArt::Zombie,Art {
        animation : blueprint::art::Animation::LoopBackForth,
        frames:[Rect2::new(16.0, 0.0, 16.0, 16.0), Rect2::new(16.0, 16.0, 16.0, 16.0)].into(),
        frames_per_second:2.0,
        height:1.0,
        width:1.0,
        texture_id:ZombieTexture::Spritesheet,
    });
  
    engine.config.window_title = "Zombie Example".into();

    let mut s = engine.world.new_sprite(ZombieArt::Player);
    s.pos.x = 0.0;
    s.pos.y = 0.0;


    engine.systems.push(update);
    engine.systems.push(draw);
    Engine::run(engine);
}