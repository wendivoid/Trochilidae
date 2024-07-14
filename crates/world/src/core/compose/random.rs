use rand::Rng;
use std::ops::Range;

use crate::{
    core::utils::random_color,
    moisture::Moisture,
    terrain::{Cell, CellColor, Elevation},
    water::WaterTable,
};

use super::ComposeCell;

pub struct RandomCellComposer {
    pub water: Range<f32>,
    pub moisture: Range<f32>,
    pub elevation: Range<f32>,
}

impl Default for RandomCellComposer {
    fn default() -> RandomCellComposer {
        RandomCellComposer {
            water: -1.0..1.0,
            moisture: -1.0..1.0,
            elevation: -1.0..1.0,
        }
    }
}

impl ComposeCell for RandomCellComposer {
    fn compose_cell(&mut self, coord: hexx::Hex) -> crate::core::bundles::CellBundle {
        let mut rng = rand::thread_rng();
        crate::core::bundles::CellBundle {
            cell: Cell(coord),
            color: CellColor(random_color()),
            elevation: Elevation(rng.gen_range(self.elevation.clone())),
            water_table: WaterTable(rng.gen_range(self.water.clone())),
            moisture: Moisture(rng.gen_range(self.moisture.clone())),
        }
    }
}
