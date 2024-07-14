use bevy_ecs::prelude::*;
use derive_more::Deref;
use hexx::Hex;

#[derive(Component, Deref)]
pub struct Chunk(pub Hex);
