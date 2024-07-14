use bevy_ecs::prelude::*;

use crate::{moisture::Moisture, terrain::*};

#[derive(Bundle, Default)]
pub struct CellBundle {
    pub cell: Cell,
    pub color: CellColor,
    pub elevation: Elevation,
    pub water_table: crate::water::WaterTable,
    pub moisture: Moisture,
}
