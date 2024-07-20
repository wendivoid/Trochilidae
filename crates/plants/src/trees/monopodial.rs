use lsystems::Value;
use crate::builder::{LSystemBuilder, Token};

use super::mesh::MeshRenderConfig;

pub fn monopodial() -> (MeshRenderConfig, lsystems::LSystem<Token>) {
    let lsys = LSystemBuilder::new("A(1,0.25)")
        .unwrap()
        .rule("A(l,w) -> F(l,w)[&(c)B(l*e,w*h)]/(135.7)A(l*b,w*h)")
        .unwrap()
        .rule("B(l,w) -> F(l,w)[-(d)$C(l*e,w*h)]C(l*b,w*h)")
        .unwrap()
        .rule("C(l,w) -> WF(l,w)[+(d)$B(l*e,w*h)]B(l*b,w*h)")
        .unwrap()
        .variable('b', Value::Num(0.9))
        .variable('e', Value::Num(0.8))
        .variable('c', Value::Num(45.0))
        .variable('d', Value::Num(45.0))
        .variable('h', Value::Num(0.707))
        .variable('W', Value::Color(0.569, 0.608, 0.196, 0.0))
        .build();

    (Default::default(), lsys)
}
