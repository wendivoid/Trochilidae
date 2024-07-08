use bevy_ecs::prelude::*;
use bevy_color::prelude::*;

#[derive(Component, Default, PartialEq, Clone)]
pub struct CellColor(pub Color);