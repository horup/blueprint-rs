use glam::Vec2;

use crate::{world::GameWorld, math::Rect2};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Animation {
    Default,
    None,
    LoopForwardBackward,
    LoopBackwardForward,
    LoopReset,
    ForwardStop,
    Stopped
}
impl Default for Animation {
    fn default() -> Self {
        Animation::None
    }
}

// TODO: refactor texture_id, frame, etc.
// TODO: refactor Rect2 into Frame type, since this will be extended in the future.
// TODO: add new_xxx which can instantiate zombie game
#[derive(Clone, Default)]
pub struct Art<W:GameWorld> {
    pub texture_id:W::Texture,
    pub frames:Vec<Rect2<f32>>,
    pub frames_per_second:f32,
    pub default_animation:Animation,
    pub origin:Vec2
}

impl<W:GameWorld> Art<W> {
    pub fn new_1x1(texture:W::Texture, frame:Rect2<f32>) -> Art<W> {
        Self {
            texture_id:texture,
            frames:[frame].into(),
            default_animation:Animation::None,
            frames_per_second:0.0,
            origin:Vec2::new(0.5, 0.5)
        }
    }
}