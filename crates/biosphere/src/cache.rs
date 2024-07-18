use bevy_ecs::prelude::*;
use bevy_utils::HashMap;
use derive_more::Deref;

use crate::phenotype::PhenotypeId;

#[derive(Resource, Default, Deref)]
pub struct PhenotypeCache {
    pub inner: HashMap<PhenotypeId, Entity>,
}
