mod bundle;
mod components;
mod material;
mod plugin;
mod systems;

pub use self::bundle::WaterBundle;
pub use self::components::*;
pub use self::material::{StandardWaterMaterial, WaterMaterial};
pub use self::plugin::WaterPlugin;

use bevy_asset::Handle;

pub const WATER_MATERIAL: Handle<StandardWaterMaterial> =
    Handle::weak_from_u128(81256742647789532081);
