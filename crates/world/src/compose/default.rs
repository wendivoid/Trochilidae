use hexx::Hex;

use super::ComposeCell;
use crate::bundles::CellBundle;

pub struct DefaultCellComposer;

impl ComposeCell for DefaultCellComposer {
    fn compose_cell(&mut self, _: Hex) -> CellBundle {
        Default::default()
    }
}