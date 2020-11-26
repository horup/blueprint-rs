use blueprint::{art::Art, context::Context, engine::Engine, event::Event, math::Rect2, world::GameWorld};
use glam::{Vec2, Vec3};
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
    BloodSplatter
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

            for s in ctx.world.sprites_iter_mut() {
                s.ext.cooldown -= delta;
                if s.ext.cooldown < 0.0 {
                    s.ext.cooldown = 0.0;
                }
            }
        },
        _ => {}
    }
}

// TODO: 1) move movement code to engine, since this can be reused
// TODO: 2) implement zombie touch
// TODO: implement AI
// TODO: implement restart of game
// TODO: implement score or similar
fn player_input_locomotion_cooldown(ctx:&mut Context<ZombieWorld>) {
    // TODO: make common in engine

    match  ctx.event {
        Event::Update(delta) => {
            if let Some(player) = ctx.world.find_sprite_mut(|x| {x.art == ZombieArt::Player}) {
                
                let k = ctx.input.keyboard;
                let x = if k.left { -1.0 } else if k.right { 1.0 } else { 0.0 };
                let y = if k.up { -1.0 } else if k.down { 1.0 } else { 0.0 };
                let max_speed = 5.0;
                let target_vel:Vec3 = Vec3::new(x, y, 0.0) * max_speed;
                let target_vel = if target_vel.length() > max_speed {target_vel.normalize() * max_speed} else {target_vel};

                player.locomotion.target_vel = target_vel;
                let target_vel = player.locomotion.target_vel;
                let vel = player.vel;
                let diff:Vec3 = target_vel - vel;
                if diff.length() > 0.0 {
                    let acceleration = 30.0 * *delta;
                    let acceleration:Vec3 = diff.normalize() * acceleration;
                    let acceleration = if acceleration.length() < diff.length() {acceleration} else {diff};
                    player.vel += acceleration;
                }
                

             /*   let loc = player.locomotion;
                println!("{}", loc.target_vel);
                let mut diff:Vec3 = (loc.target_vel - player.vel) * *delta;
                let acceleration_max:Vec3 = player.locomotion.acceleration_max * *delta;*/
            /*    diff.x = if diff.x.abs() > acceleration_max.x {acceleration_max.x * diff.x.signum()} else {diff.x};
                diff.y = if diff.y.abs() > acceleration_max.y {acceleration_max.y * diff.y.signum()} else {diff.y};
                diff.z = if diff.z.abs() > acceleration_max.x {acceleration_max.z * diff.z.signum()} else {diff.z};*/



                if player.ext.cooldown <= 0.0 && ctx.input.mouse.primary {
                    let pos = player.pos;
                    let target = ctx.input.mouse.pos;
                    let v = target - pos;
                    let v = v.normalize();

                    player.ext.cooldown = 0.5;
                    player.ext.health = 5.0;
                    
                    let ball = ctx.world.new_sprite(ZombieArt::Ball);
                    ball.pos = pos + v * 1.1;
                    ball.vel = v * 4.0;
                }
            }
        },
        _ => {}
    }
}

fn on_collision(ctx:&mut Context<ZombieWorld>) {
    if let Event::CollisionBetweenSprites(id1, id2) = ctx.event {
        let sprite1 = ctx.world.get_sprite(id1);
        let sprite2 = ctx.world.get_sprite(id2);
        if let (Some(sprite1), Some(sprite2)) = (sprite1, sprite2) {
            if sprite1.art == ZombieArt::Ball {
                // TODO: spawn an effect to show splash
                ctx.world.delete_sprite(id1);
                if let Some(sprite2) = ctx.world.get_sprite_mut(id2) {
                    sprite2.ext.health -= 1.0;
                    if sprite2.ext.health <= 0.0 {
                        ctx.world.delete_sprite(id2);
                    }
                }
            }
        }
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
        origin:Vec2::new(0.5, 0.5)
    });
    engine.art.insert(ZombieArt::Zombie,Art {
        animation : blueprint::art::Animation::LoopBackForth,
        frames:[Rect2::new(16.0, 0.0, 16.0, 16.0), Rect2::new(16.0, 16.0, 16.0, 16.0)].into(),
        frames_per_second:2.0,
        height:1.0,
        width:1.0,
        texture_id:ZombieTexture::Spritesheet,
        origin:Vec2::new(0.5, 0.5)
    });

    // TODO: refactor into a function similar to new_1x1
    engine.art.insert(ZombieArt::BloodSplatter,Art {
        animation : blueprint::art::Animation::Loop,
        frames:[Rect2::new(0.0, 32.0, 32.0, 32.0), Rect2::new(32.0, 32.0, 32.0, 32.0), Rect2::new(64.0, 32.0, 32.0, 32.0)].into(),
        frames_per_second:10.0,
        height:0.5,
        width:0.5,
        texture_id:ZombieTexture::Spritesheet,
        origin:Vec2::new(0.5, 0.5)
    });
    engine.art.insert(ZombieArt::Ball, Art::new_1x1(ZombieTexture::Spritesheet, Rect2::new(32.0, 0.0, 16.0, 16.0)));
  
    engine.config.window_title = "Zombie Example".into();

    let mut s = engine.world.new_sprite(ZombieArt::Player);
    s.pos.x = 0.0;
    s.pos.y = 0.0;

    let mut test = engine.world.new_sprite(ZombieArt::BloodSplatter);
    test.clip = blueprint::sprite::Clip::None;


    engine.systems.push(update);
    engine.systems.push(player_input_locomotion_cooldown);
    engine.systems.push(on_collision);
    Engine::run(engine);
}