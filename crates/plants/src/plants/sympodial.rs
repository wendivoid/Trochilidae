use lsystems::Value;
use crate::{PlantSystem, builder::PlantBuilder};

pub fn sympodial() -> PlantSystem {
    PlantBuilder::new("A(1,0.25)").unwrap()
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