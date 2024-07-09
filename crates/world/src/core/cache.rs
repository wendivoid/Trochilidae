use hexx::Hex;
use bevy_utils::HashMap;
use bevy_ecs::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct MeshCache {
    pub inner: HashMap<Hex, Entity>
}