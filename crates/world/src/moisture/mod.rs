mod plugin;
mod bundle;
mod systems;
mod components;

pub use self::components::*;
pub use self::bundle::MoistureBundle;
pub use self::plugin::MoisturePlugin;

use bevy_asset::Handle;
use bevy_pbr::StandardMaterial;

pub const MOISTURE_MATERIAL: Handle<StandardMaterial> = Handle::weak_from_u128(22256346347189579583);