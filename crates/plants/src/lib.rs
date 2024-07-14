mod builder;
mod bundle;
mod plugin;

pub mod material;
pub mod mesh;
pub mod plants;

use bevy_asset::Handle;
use bevy_render::render_resource::Shader;

pub use self::bundle::PlantBundle;
pub use self::plugin::PlantPlugin;

pub type PlantSystem = lsystems::LSystem<builder::Token>;

pub const PLANT_FRAGMENT_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(53274375143789590502);
pub const PLANT_VERTEX_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(53976175143789576541);
pub const PLANT_PREPASS_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(78349375173749520502);
