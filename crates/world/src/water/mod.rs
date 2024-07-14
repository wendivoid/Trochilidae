mod plugin;
mod bundle;
mod systems;
mod components;

pub use self::components::*;
pub use self::bundle::WaterBundle;
pub use self::plugin::WaterPlugin;

use bevy_asset::Handle;
use bevy_pbr::StandardMaterial;

pub const WATER_MATERIAL: Handle<StandardMaterial> = Handle::weak_from_u128(81256742647789532081);