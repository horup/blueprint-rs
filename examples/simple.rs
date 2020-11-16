use std::time::Duration;

use blueprint::{engine::Engine, world::World};

fn main() {
    let initial:World<f32,f32> = World::default();
    Engine::run("hello world", initial);
}