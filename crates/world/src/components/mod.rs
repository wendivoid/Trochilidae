mod cell;
mod chunk;
mod color;
mod elevation;
mod mesh_handle;

pub use self::cell::Cell;
pub use self::chunk::Chunk;
pub use self::color::CellColor;
pub use self::elevation::Elevation;
pub use self::mesh_handle::MeshHandle;

use bevy_ecs::prelude::*;

#[derive(Component, Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct HexWorld;

#[derive(Component, Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct Observer;

#[derive(Component, Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct ViewPort;

#[derive(Component, Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct Canvas;