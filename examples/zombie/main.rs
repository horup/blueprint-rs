use blueprint::{art::{Animation, Art}, context::Context, engine::Engine, event::Event, math::Rect2, world::{GameWorld, World}};
use glam::{Vec2, Vec3};
use rand::random;


#[derive(Clone)]
enum ZombieWorldState {
    Ready,
    InProgress,
    End
}

#[derive(Clone)]
struct ZombieWorld {
    pub timer:f32,
    pub state:ZombieWorldState
}


impl GameWorld for ZombieWorld {
    type Sprite = ZombieSprite;
    type Event = ZombieEvent;
    type Art = ZombieArt;
    type Texture = ZombieTexture;
}

impl ZombieWorld {
    pub fn new_game(world:&mut World<ZombieWorld>) {
        world.ext.timer = 0.0;
        world.new_sprite(ZombieArt::Ready).size.x = 4.0;
    }
}

#[derive(Debug, Copy, Clone)]
struct ZombieSprite {
    cooldown:f32,
    health:f32
}

impl Default for ZombieSprite {
    fn default() -> Self {
        Self {
            cooldown:0.0,
            health: 2.0
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum ZombieEvent {
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum ZombieArt {
    Player,
    Zombie,
    Ball,
    Ready,
    E(Effect)
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Effect {
    BloodSplatter
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum ZombieTexture {
    Spritesheet
}

impl Default for ZombieWorld {
    fn default() -> Self {
        Self {
            timer:0.0,
            state:ZombieWorldState::Ready
        }
    }
}

fn timer_and_cooldown_update(ctx:&mut Context<ZombieWorld>) 
{
    match ctx.event {
        blueprint::event::Event::Update(delta) => {
            let timer = &mut ctx.world.ext.timer;
            *timer += delta;

            match ctx.world.ext.state {
                ZombieWorldState::Ready => {
                    
                }
                ZombieWorldState::InProgress => {
                    if *timer > 0.2 {
                        *timer = 0.0;
                        let mut s = ctx.world.new_sprite(ZombieArt::Zombie);
                        let dy = 8.0;
                        let dx = 16.0;
                        s.pos.x = random::<f32>() * dx - dx / 2.0;
                        s.frame = s.pos.x % 10.0;
                        s.pos.y = -dy;
                        s.locomotion.target_vel = Vec3::new(0.0, 1.0, 0.0);
                    }
        
                    for s in ctx.world.sprites_iter_mut() {
                        s.ext.cooldown -= delta;
                        if s.ext.cooldown < 0.0 {
                            s.ext.cooldown = 0.0;
                        }
        
                        if let ZombieArt::E(effect) = s.art {
                            if s.animation == Animation::Stopped {
                                s.delete();
                            }
                        }
                    }
                }
                ZombieWorldState::End => {}
            }
        },
        _ => {}
    }
}

// TODO: 1) implement zombie attack center
// TODO: 2) implement zombie touch
// TODO: implement AI
// TODO: implement restart of game
// TODO: implement score or similar
fn player_input_update(ctx:&mut Context<ZombieWorld>) {
    match  ctx.event {
        Event::Update(_delta) => {
            if let Some(player) = ctx.world.find_sprite_mut(|x| {x.art == ZombieArt::Player}) {
                
                let k = ctx.input.keyboard;
                let x = if k.left { -1.0 } else if k.right { 1.0 } else { 0.0 };
                let y = if k.up { -1.0 } else if k.down { 1.0 } else { 0.0 };
                let max_speed = 5.0;
                let target_vel:Vec3 = Vec3::new(x, y, 0.0) * max_speed;
                let target_vel = if target_vel.length() > max_speed {target_vel.normalize() * max_speed} else {target_vel};

                player.locomotion.target_vel = target_vel;

                if player.ext.cooldown <= 0.0 && ctx.input.mouse.primary {
                    let pos = player.pos;
                    let target = ctx.input.mouse.pos;
                    let v = target - pos;
                    let v = v.normalize();

                    player.ext.cooldown = 0.25;
                    
                    let ball = ctx.world.new_sprite(ZombieArt::Ball);
                    ball.pos = pos + v * 1.1;
                    ball.vel = v * 8.0;
                    ball.locomotion.target_vel = ball.vel;
                }
            }
        },
        _ => {}
    }
}

fn collision_update(ctx:&mut Context<ZombieWorld>) {
    if let Event::CollisionBetweenSprites(id1, id2) = ctx.event {
        let sprite1 = ctx.world.get_sprite(id1);
        let sprite2 = ctx.world.get_sprite(id2);
        if let (Some(sprite1), Some(sprite2)) = (sprite1, sprite2) {
            if sprite1.art == ZombieArt::Ball {
                let pos1 = sprite1.pos;
                ctx.world.delete_sprite(id1);
                if let Some(sprite2) = ctx.world.get_sprite_mut(id2) {
                    sprite2.ext.health -= 1.0;

                    if sprite2.ext.health <= 0.0 {
                        ctx.world.delete_sprite(id2);
                    } else {
                        let v:Vec3 = sprite2.pos - pos1;
                        let v = v.normalize();
                        sprite2.vel += v * 10.0;
                    }

                    let mut splatter = ctx.world.new_sprite(ZombieArt::E(Effect::BloodSplatter));
                    splatter.size = [2.0, 2.0].into();
                    splatter.clip = blueprint::sprite::Clip::None;
                    splatter.pos = pos1;
                }
            }
        }
    }
}

fn main() {
    let mut engine:Engine<ZombieWorld> = Engine::new("Zombie Example".into());
    engine.camera.zoom = 32.0;
    engine.load_texture(include_bytes!("spritesheet.png"), ZombieTexture::Spritesheet);
    engine.art.insert(ZombieArt::Player, Art {
        default_animation : blueprint::art::Animation::LoopForwardBackward,
        frames:[Rect2::new(0.0, 0.0, 16.0, 16.0), Rect2::new(0.0, 16.0, 16.0, 16.0)].into(),
        frames_per_second:2.0,
        texture:ZombieTexture::Spritesheet,
        origin:Vec2::new(0.5, 0.5)
    });
    engine.art.insert(ZombieArt::Zombie,Art {
        default_animation : blueprint::art::Animation::LoopForwardBackward,
        frames:[Rect2::new(16.0, 0.0, 16.0, 16.0), Rect2::new(16.0, 16.0, 16.0, 16.0)].into(),
        frames_per_second:2.0,
        texture:ZombieTexture::Spritesheet,
        origin:Vec2::new(0.5, 0.5)
    });

    // TODO: refactor into a function similar to new_1x1
    engine.art.insert(ZombieArt::E(Effect::BloodSplatter),Art {
        default_animation : blueprint::art::Animation::ForwardStop,
        frames:[Rect2::new(0.0, 32.0, 32.0, 32.0), Rect2::new(32.0, 32.0, 32.0, 32.0), Rect2::new(64.0, 32.0, 32.0, 32.0)].into(),
        frames_per_second:10.0,
        texture:ZombieTexture::Spritesheet,
        origin:Vec2::new(0.5, 0.5)
    });

    engine.art.insert(ZombieArt::Ready, Art::new_1x1(ZombieTexture::Spritesheet, Rect2::new(0.0, 64.0, 32.0, 8.0)));
    engine.art.insert(ZombieArt::Ball, Art::new_1x1(ZombieTexture::Spritesheet, Rect2::new(32.0, 0.0, 16.0, 16.0)));
    engine.systems.push(timer_and_cooldown_update);
    engine.systems.push(player_input_update);
    engine.systems.push(collision_update);

    ZombieWorld::new_game(&mut engine.world);

    Engine::run(engine);
}