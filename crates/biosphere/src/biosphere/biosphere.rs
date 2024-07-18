use bevy_ecs::prelude::*;

use super::{Ancestry, PhenotypeRegistry};

#[derive(Default, Resource)]
pub struct BioSphere {
    pub graph: Ancestry,
    pub registry: PhenotypeRegistry,
}

impl BioSphere {
    pub fn with_defaults() -> BioSphere {
        let mut registry = PhenotypeRegistry::default();
        let mono_id = registry.add(crate::creatures::monopodial());
        let sym_id = registry.add(crate::creatures::sympodial());

        let mut graph = Ancestry::default();
        graph.add_node(mono_id);
        graph.add_node(sym_id);
        BioSphere { registry, graph }
    }
}
