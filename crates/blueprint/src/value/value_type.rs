use std::rc::Rc;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum ValueType {
    Any,
    Bool,
    Int,
    Float,
    Double,
    String,
    Dictionary,
    List,
    Vec2(Rc<ValueType>),
    Vec3(Rc<ValueType>),
    Vec4(Rc<ValueType>),
    Tuple(Rc<ValueType>, Rc<ValueType>),
}
