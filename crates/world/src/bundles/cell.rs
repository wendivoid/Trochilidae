use bevy_ecs::prelude::*;

use crate::components::*;

#[derive(Bundle, Default)]
pub struct CellBundle {
    pub cell: Cell,
    pub color: CellColor,
    pub elevation: Elevation,
}