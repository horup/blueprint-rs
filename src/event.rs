
pub enum Event<T> {
    Tick(f32),
    Game(T)
}