use glam::Vec2;


#[derive(Debug, Copy, Clone, Default)]
pub struct Keyboard {
    pub left:bool,
    pub right:bool,
    pub up:bool,
    pub down:bool
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Mouse {
    pub pos:Vec2,
    pub primary:bool
}

// TODO: implement generics support
#[derive(Debug, Copy, Clone, Default)]
pub struct Input {
    pub keyboard:Keyboard,
    pub mouse:Mouse
}