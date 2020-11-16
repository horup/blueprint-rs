use blueprint::{context::Context, engine::Engine};


fn tick(ctx:&mut Context<(),(),()>) 
{

}

fn main() {
    let mut engine:Engine<(),(),()> = Engine::new();
    engine.systems_mut().push(|x| {
    });

    engine.systems_mut().push(tick);
    Engine::run(engine);
}