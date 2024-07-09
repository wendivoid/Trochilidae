use hexx::Hex;
use bevy_ecs::prelude::*;

#[derive(Component, Debug, Default, PartialEq, Clone)]
pub struct Cell(pub Hex);