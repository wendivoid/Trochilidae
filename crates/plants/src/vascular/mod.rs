pub mod builder;
pub mod mesh;
pub mod render;

mod monopodial;
mod sympodial;

pub use self::monopodial::monopodial;
pub use self::sympodial::sympodial;

pub type VascularLSystem = lsystems::LSystem<builder::Token>;
