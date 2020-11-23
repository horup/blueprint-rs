use glam::*;

use crate::{world::GameWorld};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct SpriteID {
    pub index:u16,
    pub generation:u16
}

impl SpriteID {
    pub fn new(index:u16) -> SpriteID {
        Self {
            index:index,
            generation:0
        }
    }
}

// TODO: Add rotation
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
    pub owner:u128
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
            owner:0
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


