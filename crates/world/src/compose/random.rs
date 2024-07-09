use rand::Rng;
use std::ops::Range;

use crate::{components::{Cell, CellColor, Elevation}, utils::random_color};

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
    fn compose_cell(&mut self, coord: hexx::Hex) -> crate::bundles::CellBundle {
        let mut rng = rand::thread_rng();
        crate::bundles::CellBundle {
            cell: Cell(coord),
            color: CellColor(random_color()),
            elevation: Elevation(rng.gen_range(self.elevation.clone())),
        }
    }
}