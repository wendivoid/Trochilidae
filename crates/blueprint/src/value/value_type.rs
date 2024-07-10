use std::rc::Rc;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum GraphValueType {
    Any,
    Bool,
    Int,
    Float,
    Double,
    String,
    Dictionary,
    List,
    Vec2(Rc<GraphValueType>),
    Vec3(Rc<GraphValueType>),
    Vec4(Rc<GraphValueType>),
    Tuple(Rc<GraphValueType>, Rc<GraphValueType>),
}
