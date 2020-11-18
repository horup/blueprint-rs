use glam::*;

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
// TODO: frame, width, etc. support
pub struct Sprite<S> {
    id:SpriteID,
    in_use:bool,
    pub pos:Vec3,
    pub visible:bool,
    pub sprite_type_id:u32,
    pub frame:f32,
    pub animation_reverse:bool,
    pub ext:S,
}

impl<S:Default> Sprite<S> {
    pub fn new(id:SpriteID) -> Self {
        Self {
            id:id,
            pos:Vec3::new(0.0, 0.0, 0.0),
            in_use:true,
            visible:false,
            sprite_type_id:0,
            frame:1.0,
            animation_reverse:false,
            ext:S::default()
        }
    }
}

impl<S> Sprite<S> {
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


