use std::iter::Filter;

use crate::sprite::{Sprite, SpriteID};


pub struct World<W, S:Default>
{
    sprites:Vec<Sprite<S>>,
    game:W
}

impl<W:Default, S:Default> Default for World<W, S> {
    fn default() -> Self {
        Self {
            sprites:Vec::new(),
            game:W::default()
        }
    }
}

impl<W, S:Default> World<W, S> {
    pub fn new_sprite(&mut self) -> &mut Sprite<S> {

        let mut free:Option<SpriteID> = None;
        for sprite in &self.sprites {
            if !sprite.in_use() {
                let mut id = *sprite.id();
                id.generation += 1;
                free = Some(id);

            }
        }

        if let None = free {
            free = Some(SpriteID {
                generation:0,
                index:self.sprites.len() as u16
            });

            self.sprites.push(Sprite::new(free.unwrap()));
        }

        let id = free.unwrap();
        let sprite = self.sprites.get_mut(id.index as usize).unwrap();
        *sprite = Sprite::new(id);
        sprite
    }

    pub fn delete_sprite(&mut self, id: SpriteID) {
        if let Some(sprite) = self.sprites.get_mut(id.index as usize) {
            sprite.delete();
        }
    }

    /*pub fn sprites(&self) -> &[Sprite<S>] {
        self.sprites.as_slice()
    }

    pub fn sprites_mut(&mut self) -> &mut [Sprite<S>] {
        self.sprites.as_mut_slice()
    }*/

    pub fn sprites_iter(&self) -> impl Iterator<Item = &Sprite<S>> {
        self.sprites.iter().filter(|x| x.in_use())
    }

    pub fn sprites_iter_mut(&mut self) -> impl Iterator<Item = &mut Sprite<S>> {
        self.sprites.iter_mut().filter(|x| x.in_use())
    }

    pub fn game(&self) -> &W {
        &self.game
    }

    pub fn game_mut(&mut self) -> &mut W {
        &mut self.game
    }
}