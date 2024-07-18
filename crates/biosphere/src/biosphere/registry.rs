use bevy_utils::HashMap;

use crate::phenotype::{Phenotype, PhenotypeId};

#[derive(Default)]
pub struct PhenotypeRegistry {
    pub inner: HashMap<PhenotypeId, Phenotype>,
}

impl PhenotypeRegistry {
    pub fn add(&mut self, p: Phenotype) -> PhenotypeId {
        let id = PhenotypeId::default();
        self.inner.insert(id, p);
        id
    }
}
