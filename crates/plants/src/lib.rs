mod token;
mod parser;
mod builder;

pub mod mesh;

pub use self::token::Token;
pub use self::builder::PlantBuilder;

pub type PlantSystem = lsystems::LSystem<Token>;

pub fn monopodial() -> PlantSystem {
    use lsystems::Value;
    PlantBuilder::new("A(1,0.25)").unwrap()
        .rule("A(l,w) -> F(l,w)[&(c)B(l*e,w*h)]/(135.7)A(l*b,w*h)").unwrap()
        .rule("B(l,w) -> F(l,w)[-(d)$C(l*e,w*h)]C(l*b,w*h)").unwrap()
        .rule("C(l,w) -> F(l,w)[+(d)$B(l*e,w*h)]B(l*b,w*h)").unwrap()
        .variable('b', Value::Num(0.9))
        .variable('e', Value::Num(0.8))
        .variable('c', Value::Num(45.0))
        .variable('d', Value::Num(45.0))
        .variable('h', Value::Num(0.707))
        .build()
}

pub fn sympodial() -> PlantSystem {
    use lsystems::Value;
    PlantBuilder::new("A(1,0.25)").unwrap()
        .rule("A(l,w) -> F(l,w)[&(c)B(l*b,w*h)]//(180)[&(d)B(l*e,w*h)").unwrap()
        .rule("B(l,w) -> F(l,w)[+(c)$B(l*b,w*h)][-(d)$B(l*e,w*h)]").unwrap()
        .variable('b', Value::Num(0.9))
        .variable('e', Value::Num(0.7))
        .variable('c', Value::Num(5.0))
        .variable('d', Value::Num(65.0))
        .variable('h', Value::Num(0.707))
        .build()
}

pub fn ternary() -> PlantSystem {
    use lsystems::Value;
    PlantBuilder::new("F(0.5,0.15)A").unwrap()
        .rule("A -> TF(0.5,1)[&(c)F(0.5,1)A]/(b)[&(c)F(0.5,1)A]/(e)[&(c)F(0.5,1)A]").unwrap()
        .rule("F(l,w) -> F(l*d,w*h)").unwrap()
        .variable('b', Value::Num(94.64))
        .variable('e', Value::Num(132.63))
        .variable('c', Value::Num(18.95))
        .variable('d', Value::Num(1.109))
        .variable('h', Value::Num(1.723))
        .build()
}

pub fn cordate() -> PlantSystem {
    use lsystems::Value;
    PlantBuilder::new("G[A][B]").unwrap()
        .rule("A -> [+A{.].C.}").unwrap()
        .rule("B -> [-B{.].C.}").unwrap()
        .rule("C -> FFFC").unwrap()
        .variable('G', Value::Color(0.0, 1.0, 0.0, 0.1))
        .build()
}

pub fn simple_leaf() -> PlantSystem {
    use lsystems::Value;
    PlantBuilder::new("{.A(0)}").unwrap()
        .rule("A(i) -> F(b,c)[-B(i).][A(i+1)][+B(i).]").unwrap()
        .rule("B(i):i>0 -> F(d,e)B(i-f)").unwrap()
        .rule("F(s,r) -> F(s*r,r)").unwrap()
        .variable('b', Value::Num(5.0))
        .variable('e', Value::Num(1.0))
        .variable('c', Value::Num(1.0))
        .variable('d', Value::Num(1.0))
        .variable('f', Value::Num(0.0))
        .build()
}

pub fn rose_leaf() -> PlantSystem {
    use lsystems::Value;
    PlantBuilder::new("[{A(0).}][{C(0).}]").unwrap()
        .rule("A(i) -> .F(b,c).[+B(i)F(f,h,i).}][+B(i){.]A(i+1)").unwrap()
        .rule("C(i) -> .F(b,c).[-B(i)F(f,h,i).}][-B(i){.]C(i+1)").unwrap()
        .rule("B(i) : i > 0 -> F(d,e)B(i-1)").unwrap()
        .rule("F(s,r) -> F(s*r,r)").unwrap()
        .rule("F(s,r,i) : i > 1 -> F(s*r,r,i-1)").unwrap()
        .variable('b', Value::Num(5.0))
        .variable('e', Value::Num(1.0))
        .variable('c', Value::Num(1.5))
        .variable('d', Value::Num(1.3))
        .variable('f', Value::Num(3.0))
        .variable('h', Value::Num(0.3))
        .build()
}