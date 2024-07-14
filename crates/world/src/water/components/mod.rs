mod handle;
mod table;

pub use self::handle::WaterMeshHandle;
pub use self::table::WaterTable;

use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct WaterEntity(pub Entity);
