use hexx::Hex;
use derive_more::Deref;
use bevy_utils::HashMap;
use bevy_ecs::prelude::*;

#[derive(Debug, Resource, Deref)]
pub struct EntityMap {
    pub inner: HashMap<Hex, Entity>,
}

impl EntityMap {
    pub fn new(inner: HashMap<Hex, Entity>) -> Self {
        EntityMap {
            inner
        }
    }
}


#[derive(Debug, Default, Resource, Deref)]
pub struct EntityCache {
    pub inner: HashMap<Hex, Entity>
}