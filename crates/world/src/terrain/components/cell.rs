use bevy_ecs::prelude::*;
use derive_more::Deref;
use hexx::Hex;

#[derive(Component, Debug, Default, PartialEq, Clone, Deref)]
pub struct Cell(pub Hex);
