mod plugin;
mod systems;
mod components;

pub use self::components::*;
pub use self::plugin::TerrainPlugin;

use bevy_asset::Handle;
use bevy_pbr::StandardMaterial;

pub const TERRAIN_MATERIAL: Handle<StandardMaterial> = Handle::weak_from_u128(81224375647789535861);