#![feature(const_type_name)]

mod descriptor;
mod entities;
mod settings;

pub mod core;
pub mod moisture;
pub mod observer;
pub mod sky;
pub mod terrain;
pub mod time;
pub mod water;

pub use self::core::WorldPlugin;
pub use self::descriptor::ChunkDescriptor;
pub use self::entities::{EntityCache, EntityMap};
pub use self::settings::WorldSettings;

#[derive(bevy_ecs::prelude::Event)]
pub struct InsertWorldChunk {
    pub entity: bevy_ecs::prelude::Entity,
    pub descriptor: ChunkDescriptor,
    pub cells: Vec<(hexx::Hex, hexx::Hex)>,
}
