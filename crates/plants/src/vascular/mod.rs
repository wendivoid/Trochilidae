pub mod mesh;
pub mod builder;

mod monopodial;
mod sympodial;

pub use self::monopodial::monopodial;
pub use self::sympodial::sympodial;

pub type VascularLSystem = lsystems::LSystem<builder::Token>;