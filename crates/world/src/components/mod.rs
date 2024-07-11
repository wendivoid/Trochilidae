mod cell;
mod chunk;
mod color;
mod elevation;
mod terrain_handle;
mod water_table;
mod water_handle;
mod moisture_handle;
mod moisture;

pub use self::cell::Cell;
pub use self::chunk::Chunk;
pub use self::color::CellColor;
pub use self::moisture::Moisture;
pub use self::elevation::Elevation;
pub use self::terrain_handle::TerrainMeshHandle;
pub use self::water_table::WaterTable;
pub use self::water_handle::{WaterMeshHandle, WaterEntity};
pub use self::moisture_handle::{MoistureMeshHandle, MoistureEntity};

use bevy_ecs::prelude::*;

#[derive(Component, Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct HexWorld;

#[derive(Component, Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct Observer;

#[derive(Component, Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct ViewPort;

#[derive(Component, Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct Canvas;