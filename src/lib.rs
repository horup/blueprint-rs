pub mod sprite;
pub mod world;
pub mod engine;
pub mod event;
pub mod context;
pub mod system;
pub mod config;
pub mod art;
pub mod math;
pub mod camera;
pub mod collection;
pub mod systems;
pub mod input;

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use crate::{ world::{GameWorld, World}};

    impl GameWorld for () {
        type Sprite = ();
        type Event = ();
        type Art = ();
        type Texture = ();
    }

    #[test]
    fn it_works() {
        
        let mut world:World<()> = World::default();
        let sprite = world.new_sprite(());
        sprite.pos = Vec3::new(0.0, 1.0, 0.0);
        let id = *sprite.id();
        assert_eq!(world.sprites_iter().count(), 1);
        assert_eq!(&world.sprites_iter().last().unwrap().pos, &Vec3::new(0.0, 1.0, 0.0));
     
        world.delete_sprite(&id);
        assert_eq!(world.sprites_iter().count(), 0);
    }
}
