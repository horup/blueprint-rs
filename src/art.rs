use glam::Vec2;

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
    pub height:f32,
    pub origin:Vec2
}

impl<W:GameWorld> Art<W> {
    pub fn new_1x1(texture:W::Texture, frame:Rect2<f32>) -> Art<W> {
        Self {
            texture_id:texture,
            frames:[frame].into(),
            animation:Animation::None,
            frames_per_second:0.0,
            width:1.0,
            height:1.0,
            origin:Vec2::new(0.5, 0.5)
        }
    }
}