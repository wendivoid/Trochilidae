mod handle;
mod table;

pub use self::table::WaterTable;
pub use self::handle::WaterMeshHandle;

use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct WaterEntity(pub Entity);