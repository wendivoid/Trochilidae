mod cache;
mod instance;
mod plugin;
mod systems;

pub mod biosphere;
pub mod creatures;
pub mod environment;
pub mod phenotype;

pub use self::instance::PhenotypeInstance;
pub use self::plugin::BioSpherePlugin;

#[derive(Debug, bevy_ecs::prelude::Component)]
pub struct BioSphereEntity;
