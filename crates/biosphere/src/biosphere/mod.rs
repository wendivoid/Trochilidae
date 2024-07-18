mod biosphere;
mod registry;

use crate::phenotype::PhenotypeId;

pub use self::biosphere::BioSphere;
pub use self::registry::PhenotypeRegistry;

pub type Ancestry = petgraph::graphmap::DiGraphMap<PhenotypeId, ()>;
