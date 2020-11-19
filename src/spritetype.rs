use crate::math::Rect2;

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
#[derive(Clone, Default)]
pub struct SpriteType {
    pub texture_id:u32,
    pub frames:Vec<Rect2<u16>>,
    pub frames_per_second:f32,
    pub animation:Animation
}