use hexx::Hex;
use bevy_utils::HashMap;
use bevy_ecs::prelude::*;

#[derive(Debug, Resource)]
pub struct EntityCache {
    pub inner: HashMap<Hex, Entity>,
}

impl EntityCache {
    pub fn new(inner: HashMap<Hex, Entity>) -> Self {
        EntityCache {
            inner
        }
    }
}