use hexx::Hex;
use derive_more::Deref;
use bevy_ecs::prelude::*;

#[derive(Component, Deref)]
pub struct Chunk(pub Hex);