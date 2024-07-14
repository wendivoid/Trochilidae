#![feature(const_type_name)]

mod sky;
mod time;
mod settings;
mod entities;
mod descriptor;
mod water;
mod terrain;
mod moisture;
mod observer;

pub mod core;

pub use self::core::WorldPlugin;
pub use self::settings::WorldSettings;
pub use self::descriptor::ChunkDescriptor;
pub use self::entities::{EntityMap, EntityCache};

#[derive(bevy_ecs::prelude::Event)]
pub struct InsertWorldChunk {
    pub entity: bevy_ecs::prelude::Entity,
    pub descriptor: ChunkDescriptor,
    pub cells: Vec<(hexx::Hex, hexx::Hex)>
}