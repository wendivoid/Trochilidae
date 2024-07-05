mod lsystem;
mod plugin;
mod bundle;

pub mod mesh;
pub mod material;

pub use self::bundle::PlantBundle;
pub use self::plugin::PlantPlugin;

pub type PlantSystem = lsystems::LSystem<lsystem::Token>;

pub fn monopodial() -> PlantSystem {
    use lsystems::Value;
    lsystem::PlantBuilder::new("A(1,0.25)").unwrap()
        .rule("A(l,w) -> F(l,w)[&(c)B(l*e,w*h)]/(135.7)A(l*b,w*h)").unwrap()
        .rule("B(l,w) -> F(l,w)[-(d)$C(l*e,w*h)]C(l*b,w*h)").unwrap()
        .rule("C(l,w) -> WF(l,w)[+(d)$B(l*e,w*h)]B(l*b,w*h)").unwrap()
        .variable('b', Value::Num(0.9))
        .variable('e', Value::Num(0.8))
        .variable('c', Value::Num(45.0))
        .variable('d', Value::Num(45.0))
        .variable('h', Value::Num(0.707))
        .variable('W', Value::Color(0.569, 0.608, 0.196, 0.0))
        .build()
}

pub fn sympodial() -> PlantSystem {
    use lsystems::Value;
    lsystem::PlantBuilder::new("A(1,0.25)").unwrap()
        .rule("A(l,w) -> F(l,w)[W&(c)B(l*b,w*h)]//(180)[&(d)B(l*e,w*h)").unwrap()
        .rule("B(l,w) -> F(l,w)[+(c)$B(l*b,w*h)][-(d)$B(l*e,w*h)]").unwrap()
        .variable('b', Value::Num(0.9))
        .variable('e', Value::Num(0.7))
        .variable('c', Value::Num(5.0))
        .variable('d', Value::Num(65.0))
        .variable('h', Value::Num(0.707))
        .variable('W', Value::Color(0.569, 0.608, 0.196, 0.0))
        .build()
}

pub fn ternary() -> PlantSystem {
    use lsystems::Value;
    lsystem::PlantBuilder::new("F(0.5,0.15)A").unwrap()
        .rule("A -> TF(0.5,1)[&(c)F(0.5,1)A]/(b)[&(c)F(0.5,1)A]/(e)[&(c)F(0.5,1)A]").unwrap()
        .rule("F(l,w) -> F(l*d,w*h)").unwrap()
        .variable('b', Value::Num(94.64))
        .variable('e', Value::Num(132.63))
        .variable('c', Value::Num(18.95))
        .variable('d', Value::Num(1.109))
        .variable('h', Value::Num(1.723))
        .build()
}
