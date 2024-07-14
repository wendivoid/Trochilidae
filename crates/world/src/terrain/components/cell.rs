use hexx::Hex;
use derive_more::Deref;
use bevy_ecs::prelude::*;

#[derive(Component, Debug, Default, PartialEq, Clone, Deref)]
pub struct Cell(pub Hex);