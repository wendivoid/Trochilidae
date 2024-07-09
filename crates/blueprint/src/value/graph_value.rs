use std::any::type_name;

use crate::{ExecutionError, Value};

pub trait GraphValue<'a>: Default {
    const NODE_NAME: &'static str = type_name::<Self>();

    const PROPERTY: &'a str;

    fn from_graph_value(value: Value) -> Result<Self, ExecutionError>;
}