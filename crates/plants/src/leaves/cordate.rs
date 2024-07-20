use crate::builder::{LSystemBuilder, Token};

use super::mesh::MeshRenderConfig;

pub fn cordate() -> (MeshRenderConfig, lsystems::LSystem<Token>) {
    let lsys = LSystemBuilder::new("[a][b]")
        .unwrap()
        .rule("a -> [+a{.].c.}")
        .unwrap()
        .rule("b -> [-b{.].c.}")
        .unwrap()
        .rule("c -> FFFc")
        .unwrap()
        .build();

    (Default::default(), lsys)
}
