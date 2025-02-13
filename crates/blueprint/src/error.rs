use crate::{graph::GraphNode, value::GraphValueType};

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum RegistrationError {
    NotUnique,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConversionError {
    pub from: GraphValueType,
    pub to: GraphValueType,
}

impl ConversionError {
    pub fn new(from: GraphValueType, to: GraphValueType) -> ConversionError {
        ConversionError { from, to }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExecutionError {
    UnknownNode(GraphNode),
    Conversion(ConversionError),
    Registration(RegistrationError),
    InvalidProperty { node: GraphNode, property: String },
    MissingProperty { node: GraphNode, property: String },
}

impl<'a> From<ConversionError> for ExecutionError {
    fn from(c: ConversionError) -> ExecutionError {
        ExecutionError::Conversion(c)
    }
}

impl<'a> From<GraphNode> for ExecutionError {
    fn from(c: GraphNode) -> ExecutionError {
        ExecutionError::UnknownNode(c)
    }
}

impl<'a> From<RegistrationError> for ExecutionError {
    fn from(c: RegistrationError) -> ExecutionError {
        ExecutionError::Registration(c)
    }
}
