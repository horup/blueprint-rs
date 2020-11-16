
pub enum Event<T> {
    Tick(u64, f32),
    Game(T)
}