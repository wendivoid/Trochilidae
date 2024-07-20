use lsystems::Value;

use crate::builder::{LSystemBuilder, Token};

use super::mesh::MeshRenderConfig;

pub fn rose_leaf() -> (MeshRenderConfig, lsystems::LSystem<Token>) {
    let lsys = LSystemBuilder::new("[{A(0).}][{C(0).}]")
        .unwrap()
        .rule("A(i) -> .F(b,c).[+B(i)F(f,h,i).}][+B(i){.]A(i+1)")
        .unwrap()
        .rule("C(i) -> .F(b,c).[-B(i)F(f,h,i).}][-B(i){.]C(i+1)")
        .unwrap()
        .rule("B(i) : i>0 -> F(d,e)B(i-1)")
        .unwrap()
        .rule("F(s,r) -> F(s*r,r)")
        .unwrap()
        .rule("F(s,r,i) : i>1 -> F(s*r,r,i-1)")
        .unwrap()
        .variable('b', Value::Num(5.0))
        .variable('c', Value::Num(1.15))
        .variable('d', Value::Num(1.3))
        .variable('e', Value::Num(1.25))
        .variable('f', Value::Num(1.09))
        .variable('h', Value::Num(1.09))
        .build();

    let cfg = MeshRenderConfig {
        angle: 60.0,
        length: 0.1,
        generation: 25,
        ..Default::default()
    };

    (cfg, lsys)
}
