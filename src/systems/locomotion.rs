use glam::Vec3;

use crate::{context::Context, event::Event, world::GameWorld};

pub fn locomotion<W:GameWorld>(ctx:&mut Context<W>) {
    if let Event::Update(delta) = ctx.event {
        for sprite in ctx.world.sprites_iter_mut() {
            let target_vel = sprite.locomotion.target_vel;
            let vel = sprite.vel;
            let diff:Vec3 = target_vel - vel;
            if diff.length() > 0.0 {
                let acceleration = 30.0 * *delta;
                let acceleration:Vec3 = diff.normalize() * acceleration;
                let acceleration = if acceleration.length() < diff.length() {acceleration} else {diff};
                sprite.vel += acceleration;
            }
        }
    }
}