use collision::{Aabb2, Discrete};
use glam::{Vec3};

use crate::{context::Context, event::Event, sprite::Sprite, world::GameWorld, sprite::SpriteID};



fn aabb2(pos:&Vec3) -> Aabb2<f32> {
    let r = 0.5;
    Aabb2::new(
        (pos.x - r, pos.y - r).into(),
        (pos.x + r, pos.y + r).into()
    )
}

// TODO: 2) implement collision event
// BUG: fix issue with scaling and sprite_type width/height
fn compute_movement<W:GameWorld>(ctx:&Context<W>, sprite:&Sprite<W>, diff:&Vec3) -> Vec3
{
    let mut res = sprite.pos;
    let max = 0.1;
    let distance = diff.length();
    if distance <= 0.0 {
        return res;
    }

    let mut remaining = distance;
    while remaining > 0.0 {
        let step = if distance < max { distance} else {max};
        let dir = diff.normalize();
        let vs = [Vec3::new(0.0, dir.y * step, 0.0), Vec3::new(dir.x * step, 0.0, 0.0)];
        for v in vs.iter()
        {
            let mut collision = false;
            let p:Vec3 = res + *v;
            for other_sprite in ctx.world.sprites_iter().filter(|e| e.id() != sprite.id()) {
                let v2 = res - other_sprite.pos;
                let v2 = v2.normalize();
                if v.dot(v2) < 0.0 {
                    
                    if aabb2(&p).intersects(&aabb2(&other_sprite.pos)) {
                        collision = true;
                        //push_event(Event::Collision(entity.id, other_entity.id));
                        break;
                    } 
                }
            }
            if !collision {
                res = p;
            }
        }

        remaining -= step;
    }

    res
}



pub fn movement<T:GameWorld>(ctx:&mut Context<T>)  {
    match ctx.event {
        Event::Update(delta) => {
            let sprites:Vec<(SpriteID, Vec3)> = ctx.world.sprites_iter().map(|x| (*x.id(), x.vel)).collect();
            for (id, vel) in sprites {
                let v = vel * *delta;
                let res = compute_movement(ctx, ctx.world.get_sprite(&id).unwrap(), &v);
                ctx.world.get_sprite_mut(&id).unwrap().pos = res;
            }
        },
        _ => {}
    }
}