#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Conditional {
    EqualTo,
    GreaterThan,
    LessThan,
}
