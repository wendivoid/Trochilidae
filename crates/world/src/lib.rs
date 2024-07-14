#![feature(const_type_name)]

mod descriptor;
mod entities;
mod moisture;
mod observer;
mod settings;
mod sky;
mod terrain;
mod time;
mod water;

pub mod core;

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
