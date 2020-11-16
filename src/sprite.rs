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

pub struct Sprite<S> {
    id:SpriteID,
    in_use:bool,
    pos:Vec3,
    texture_id:u16,
    visible:bool,
    game:S,
}

impl<S:Default> Sprite<S> {
    pub fn new(id:SpriteID) -> Self {
        Self {
            id:id,
            pos:Vec3::new(0.0, 0.0, 0.0),
            texture_id:0,
            in_use:true,
            visible:false,
            game:S::default()
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

    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    pub fn pos_mut(&mut self) -> &mut Vec3 {
        &mut self.pos
    }

    pub fn in_use(&self) -> bool {
        self.in_use
    }
}


