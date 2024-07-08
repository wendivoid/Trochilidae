use crate::bundles::CellBundle;

pub trait ComposeCell {
    fn compose_cell(&mut self, coord: hexx::Hex) -> CellBundle;
}