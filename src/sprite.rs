use glam::*;

use crate::{world::GameWorld};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct SpriteID {
    pub index:u16,
    pub generation:u16
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Clip {
    Default,
    None
}

impl Default for Clip {
    fn default() -> Self {
        Clip::Default
    }
}

impl SpriteID {
    pub fn new(index:u16) -> SpriteID {
        Self {
            index:index,
            generation:0
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Locomotion {
    pub target_vel:Vec3,
    pub acceleration_max:Vec3
}

impl Default for Locomotion {
    fn default() -> Self {
        Self {
            target_vel:Vec3::default(),
            acceleration_max:Vec3::splat(10.0)
        }
    }
}

// TODO: Add rotation
// TODO: added clipping type to control collision
// BUG: fixed draw order, maybe using z 
#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Sprite<W:GameWorld> {
    id:SpriteID,
    in_use:bool,
    pub pos:Vec3,
    pub vel:Vec3,
    pub scale:Vec2,
    pub visible:bool,
    pub art:W::Art,
    pub frame:f32,
    pub animation_reverse:bool,
    pub ext:W::Sprite,
    pub owner:u128,
    pub clip:Clip,
    pub locomotion:Locomotion
}

impl<W:GameWorld> Sprite<W> {
    pub fn new(id:SpriteID, art:W::Art) -> Self {
        Self {
            id:id,
            pos:Vec3::new(0.0, 0.0, 0.0),
            vel:Vec3::new(0.0, 0.0, 0.0),
            in_use:true,
            visible:false,
            art:art,
            scale:Vec2::new(1.0, 1.0),
            frame:1.0,
            animation_reverse:false,
            ext:W::Sprite::default(),
            owner:0,
            clip:Clip::default(),
            locomotion:Locomotion::default()
        }
    }
}

impl<W:GameWorld> Sprite<W> {
    pub fn id(&self) -> &SpriteID {
        &self.id
    }

    pub fn delete(&mut self) {
        self.in_use = false;
    }

    pub fn in_use(&self) -> bool {
        self.in_use
    }
}


