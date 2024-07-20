mod anatomy;
mod habitability;
mod id;

pub use self::anatomy::Anatomy;
pub use self::habitability::Habitability;
pub use self::id::PhenotypeId;

pub struct Phenotype {
    pub lifespan: f32,
    pub anatomy: Anatomy,
    pub habitability: Habitability,
}