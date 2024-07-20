use lsystems::Value;

use crate::builder::{LSystemBuilder, Token};

use super::mesh::MeshRenderConfig;

pub fn simple() -> (MeshRenderConfig, lsystems::LSystem<Token>) {
    let lsys = LSystemBuilder::new("{.A(0)}")
        .unwrap()
        .rule("A(i) -> F(b,c)[-B(i).][A(i+1)][+B(i).]")
        .unwrap()
        .rule("B(i):i>0 -> F(d,e)B(i-f)")
        .unwrap()
        .rule("F(s,r) -> F(s*r,r)")
        .unwrap()
        .variable('b', Value::Num(5.0))
        .variable('c', Value::Num(1.1))
        .variable('d', Value::Num(1.0))
        .variable('e', Value::Num(1.2))
        .variable('f', Value::Num(1.0))
        .build();

    let cfg = MeshRenderConfig {
        angle: 60.0,
        length: 0.1,
        generation: 20,
        ..Default::default()
    };

    (cfg, lsys)
}
