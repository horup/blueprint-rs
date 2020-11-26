use glam::Vec3;


#[derive(Debug, Copy, Clone, Default)]
pub struct Keyboard {
    pub left:bool,
    pub right:bool,
    pub up:bool,
    pub down:bool
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Mouse {
    pub pos:Vec3,
    pub primary:bool
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Input {
    pub keyboard:Keyboard,
    pub mouse:Mouse
}