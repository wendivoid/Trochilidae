use uuid::Uuid;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Hash, Eq, Ord)]
pub struct PhenotypeId(Uuid);

impl PhenotypeId {
    pub const NULL: PhenotypeId = PhenotypeId(Uuid::nil());
}

impl Default for PhenotypeId {
    fn default() -> PhenotypeId {
        PhenotypeId(Uuid::new_v4())
    }
}
