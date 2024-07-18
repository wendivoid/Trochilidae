use bevy_ecs::prelude::*;
use derive_more::{DerefMut, Deref};
use hexx::Hex;

#[derive(Component, Debug, Default, PartialEq, Clone, Deref, DerefMut)]
pub struct Cell(pub Hex);
