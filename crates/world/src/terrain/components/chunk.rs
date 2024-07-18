use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};
use hexx::Hex;

#[derive(Component, Deref, DerefMut)]
pub struct Chunk(pub Hex);
