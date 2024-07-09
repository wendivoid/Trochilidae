use bevy_ecs::prelude::*;
use hexx::Hex;

use crate::bundles::CellBundle;

use super::{ComposeCell, RandomCellComposer};

#[derive(Resource)]
pub struct CellComposer(pub Box<dyn ComposeCell + Sync + Send>);

impl CellComposer {
    pub fn new<C: ComposeCell + Sync + Send + 'static>(composer: C) -> CellComposer {
        CellComposer(Box::new(composer))
    }

    pub fn compose_cell(&mut self, coord: Hex) -> CellBundle {
        self.0.compose_cell(coord)
    }
}

impl Default for CellComposer {
    fn default() -> CellComposer {
        CellComposer(Box::new(RandomCellComposer::default()))
    }
}