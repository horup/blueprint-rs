use crate::{world::GameWorld, math::Rect2};

#[derive(Debug, Copy, Clone)]
pub enum Animation {
    None,
    Loop,
    LoopBackForth,
    ForwardStop
}
impl Default for Animation {
    fn default() -> Self {
        Animation::None
    }
}

// TODO: added origin
#[derive(Clone, Default)]
pub struct Art<W:GameWorld> {
    pub texture_id:W::Texture,
    pub frames:Vec<Rect2<f32>>,
    pub frames_per_second:f32,
    pub animation:Animation,
    pub width:f32,
    pub height:f32
}