use crate::phenotype::PhenotypeId;
use bevy_ecs::prelude::*;

#[derive(Component, Clone, Default)]
pub struct PhenotypeInstance {
    pub id: PhenotypeId,
    pub scale: f32,
    pub birthdate: f32,
}
