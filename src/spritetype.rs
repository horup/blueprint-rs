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
    pub animation_speed_ps:f32,
    pub animation:Animation
}