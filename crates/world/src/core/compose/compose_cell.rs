use hexx::Hex;

use crate::core::bundles::CellBundle;

pub trait ComposeCell {
    fn compose_cell(&mut self, coord: Hex) -> CellBundle;
}
