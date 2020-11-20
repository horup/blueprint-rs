use std::hash::Hash;

use glam::*;

use crate::{world::GameWorld};

#[derive(Debug, Copy, Clone)]
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

// TODO: Refactor 
// TODO: Add rotation
pub struct Sprite<W:GameWorld> {
    id:SpriteID,
    in_use:bool,
    pub pos:Vec3,
    pub scale:Vec2,
    pub visible:bool,
    pub sprite_type_id:W::Art,
    pub frame:f32,
    pub animation_reverse:bool,
    pub ext:W::Sprite,
}

impl<W:GameWorld> Sprite<W> {
    pub fn new(id:SpriteID, sprite_type:W::Art) -> Self {
        Self {
            id:id,
            pos:Vec3::new(0.0, 0.0, 0.0),
            in_use:true,
            visible:false,
            sprite_type_id:sprite_type,
            scale:Vec2::new(1.0, 1.0),
            frame:1.0,
            animation_reverse:false,
            ext:W::Sprite::default()
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


