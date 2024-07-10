#![feature(const_type_name)]

mod sky;
mod time;
mod core;
mod mesh;
mod utils;
mod debug;
mod plugin;
mod systems;
pub mod bundles;
pub mod compose;
pub mod components;

pub use self::plugin::WorldPlugin;
pub use self::core::{WorldOrigin, EntityCache, WorldSettings};

use bevy_asset::Handle;
use bevy_pbr::StandardMaterial;

pub const CHUNK_MATERIAL: Handle<StandardMaterial> = Handle::weak_from_u128(81224375647789535861);