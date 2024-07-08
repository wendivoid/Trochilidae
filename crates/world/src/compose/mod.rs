mod random;
mod compose_cell;
mod composer;

pub use self::random::RandomCellComposer;
pub use self::compose_cell::ComposeCell;
pub use self::composer::CellComposer;

pub struct DefaultCellComposer;

impl ComposeCell for DefaultCellComposer {
    fn compose_cell(&mut self, _: hexx::Hex) -> crate::bundles::CellBundle {
        Default::default()
    }
}

