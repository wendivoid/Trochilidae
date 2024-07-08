use bevy_ecs::prelude::*;

#[derive(Component, Debug, Default, PartialEq, Clone)]
pub struct Cell(pub hexx::Hex);