use collision::{Aabb2, Discrete};
use glam::{Vec3};

use crate::{context::Context, event::Event, sprite::Sprite, sprite::{Clip, SpriteID}, world::{GameWorld, World}};



fn aabb2(pos:&Vec3) -> Aabb2<f32> {
    let r = 0.5;
    Aabb2::new(
        (pos.x - r, pos.y - r).into(),
        (pos.x + r, pos.y + r).into()
    )
}

// TODO: refactor using interiour mutability 
fn compute_movement<W:GameWorld>(world:&World<W>, sprite:&Sprite<W>, diff:&Vec3, push_event:&mut dyn FnMut(Event<W::Event>)) -> Vec3
{
    let mut res = sprite.pos;
    let max = 0.1;
    let distance = diff.length();
    if distance <= 0.0 {
        return res;
    }

    let mut remaining = distance;
    let mut iterations = 0;
    let max_iterations = 10;
    while remaining > 0.0 && iterations < max_iterations {
        let step = if distance < max { distance} else {max};
        let dir = diff.normalize();
        let vs = [Vec3::new(0.0, dir.y * step, 0.0), Vec3::new(dir.x * step, 0.0, 0.0)];
        for v in vs.iter()
        {
            if sprite.clip == Clip::Default {
                let mut collision = false;
                let p:Vec3 = res + *v;
                for other_sprite in world.sprites_iter().filter(|e| e.id() != sprite.id() && e.clip != Clip::None) {
                    let v2 = res - other_sprite.pos;
                    let v2 = v2.normalize();
                    if v.dot(v2) < 0.0 {
                        
                        if aabb2(&p).intersects(&aabb2(&other_sprite.pos)) {
                            collision = true;
                            push_event(Event::CollisionBetweenSprites(*sprite.id(), *other_sprite.id()));
                            break;
                        } 
                    }
                }
                if !collision {
                    res = p;
                }
            }
        }

        remaining -= step;
        iterations += 1;
    }

    res
}



pub fn movement<T:GameWorld>(ctx:&mut Context<T>)  {
    match ctx.event {
        Event::Update(delta) => {
            let sprites:Vec<(SpriteID, Vec3)> = ctx.world.sprites_iter().map(|x| (*x.id(), x.vel)).collect();
            for (id, vel) in sprites {
                let v = vel * *delta;
                let res = compute_movement(&ctx.world, ctx.world.get_sprite(&id).unwrap(), &v, &mut ctx.push_event);
                ctx.world.get_sprite_mut(&id).unwrap().pos = res;
            }
        },
        _ => {}
    }
}