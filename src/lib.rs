pub mod sprite;
pub mod world;
pub mod engine;
pub mod event;
pub mod context;
pub mod system;
pub mod config;

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use crate::world::World;

    #[test]
    fn it_works() {
        
        let mut world:World<(), ()> = World::default();
        let sprite = world.new_sprite();
        *sprite.pos_mut() = Vec3::new(0.0, 1.0, 0.0);
        let id = *sprite.id();
        assert_eq!(world.sprites_iter().count(), 1);
        assert_eq!(world.sprites_iter().last().unwrap().pos(), &Vec3::new(0.0, 1.0, 0.0));
     
        world.delete_sprite(id);
        assert_eq!(world.sprites_iter().count(), 0);
    }
}
