use std::{hash::Hash, collections::HashMap};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Key<EngineKey:Clone + Eq + PartialEq, GameKey:Clone + Eq + PartialEq> {
    Engine(EngineKey),
    Game(GameKey)
}

#[derive(Clone)]
pub struct Collection<SubKey:Clone + Eq, CoreKey:Clone + Eq, V>{
    hashmap:HashMap<Key<SubKey, CoreKey>, V>
}
/*
impl<SubKey:Clone + Eq + PartialEq + Hash, CoreKey:Clone + Eq + PartialEq + Hash>  Default for Collection<SubKey, CoreKey> {
    fn default() -> Self {
        let mut hashmap = HashMap::new();
        hashmap.insert(Key::Unknown,  SpriteType {
            texture_id:0,
            frames:Vec::from([Rect2::new(0.0, 0.0, 16.0, 16.0), Rect2::new(16.0, 0.0, 16.0, 16.0)]),
            animation:crate::spritetype::Animation::LoopBackForth,
            frames_per_second:1.0,
            width:16.0,
            height:16.0
        });
        Self {
            hashmap
        }
    }

    
}*/

impl<EngineKey:Copy + Clone + Eq + PartialEq + Hash, GameKey:Clone + Eq + PartialEq + Hash + Copy, V> Collection<EngineKey, GameKey, V> {
    pub fn new(hashmap:HashMap<Key<EngineKey, GameKey>, V>) -> Self {
        Self {
            hashmap
        }
    }
    pub fn insert_game(&mut self, game_key:GameKey, value:V) {
        self.hashmap.insert(Key::Game(game_key), value);
    }

    pub fn get(&self, key:&Key<EngineKey, GameKey>) -> Option<&V> {
        self.hashmap.get(key)
    }

    pub fn get_game(&self, gamekey:&GameKey) -> Option<&V> {
        self.hashmap.get(&Key::Game(*gamekey))
    }

    pub fn get_game_mut(&self, gamekey:&GameKey) -> Option<&V> {
        self.hashmap.get(&Key::Game(*gamekey))
    }

    pub fn get_engine(&self, enginekey:&EngineKey) -> Option<&V> {
        self.hashmap.get(&Key::Engine(*enginekey))
    }
}




