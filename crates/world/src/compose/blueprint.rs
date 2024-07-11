use bevy_utils::tracing::error;
use blueprint::nodes;
use hexx::Hex;

use crate::bundles::CellBundle;
use crate::components::*;
use crate::compose::ComposeCell;

pub struct BlueprintComposer(pub blueprint::Blueprint);

impl BlueprintComposer {
    pub fn new(inner: blueprint::Blueprint) -> BlueprintComposer {
        BlueprintComposer(inner)
    }
}

impl ComposeCell for BlueprintComposer {
    fn compose_cell(&mut self, coord: Hex) -> CellBundle {
        self.0.rm_node::<nodes::Coord>();
        if let Err(err) = self.0.add_node(
            std::any::type_name::<nodes::Coord>(),
            nodes::Coord(coord),
        ) {
            error!("Failed to change Coordinate in the graph: {err:?}");
        }
        CellBundle {
            cell: Cell(coord),
            elevation: self.0.retrieve_or_default::<Elevation>(),
            color: self.0.retrieve_or_default::<CellColor>(),
            water_table: self.0.retrieve_or_default::<WaterTable>(),
            moisture: self.0.retrieve_or_default::<Moisture>()
        }
    }
}
