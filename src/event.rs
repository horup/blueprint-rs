use crate::sprite::SpriteID;


pub enum Event<T> {
    Update(f32),
    Draw(f32),
    CollisionBetweenSprites(SpriteID, SpriteID),
    Ext(T)
}