use hexx::Hex;
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Chunk(pub Hex);