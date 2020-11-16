use blueprint::engine::Engine;


fn main() {
    let mut engine:Engine<(),(),()> = Engine::new();
    Engine::run(engine);
}