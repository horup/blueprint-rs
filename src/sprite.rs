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
    pos:Vec3,
    visible:bool,
    sprite_type_id:u32,
    frame:f32,
    animation_reverse:bool,
    game:S,
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

    pub fn frame(&self) -> &f32 {
        &self.frame
    }

    pub fn frame_mut(&mut self) -> &mut f32 {
        &mut self.frame
    }

    pub fn sprite_type_id(&self) -> u32 {
        self.sprite_type_id
    }

    pub fn pos_mut(&mut self) -> &mut Vec3 {
        &mut self.pos
    }

    pub fn in_use(&self) -> bool {
        self.in_use
    }

    pub fn ext(&self) -> &S {
        &self.game
    }

    pub fn ext_mut(&mut self) -> &mut S {
        &mut self.game
    }
}


