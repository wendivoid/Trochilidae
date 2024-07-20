use lsystems::LSystem;

#[derive(Clone)]
pub enum Anatomy {
    Vascular(LSystem<plants::Token>),
}
