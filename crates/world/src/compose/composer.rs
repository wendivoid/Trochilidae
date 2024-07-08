use bevy_ecs::prelude::*;

use super::{ComposeCell, RandomCellComposer};

#[derive(Resource)]
pub struct CellComposer(pub Box<dyn ComposeCell + Sync + Send>);

impl CellComposer {
    pub fn new<C: ComposeCell + Sync + Send + 'static>(composer: C) -> CellComposer {
        CellComposer(Box::new(composer))
    }
}

impl Default for CellComposer {
    fn default() -> CellComposer {
        CellComposer(Box::new(RandomCellComposer::default()))
    }
}