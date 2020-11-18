#[derive(Debug, Copy, Clone)]
pub struct Rect2<T> {
    pub x:T,
    pub y:T,
    pub w:T,
    pub h:T
}

impl<T> Rect2<T> {
    pub fn new(x:T, y:T, w:T, h:T) -> Self {
        Self {x, y, w, h}
    }
}