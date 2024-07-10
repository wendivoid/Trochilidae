use std::any::type_name;

use crate::{ExecutionError, GraphValue};

pub trait FromGraphValue<'a>: Default {
    const NODE_NAME: &'static str = type_name::<Self>();

    const PROPERTY: &'a str;

    fn from_graph_value(value: GraphValue) -> Result<Self, ExecutionError>;
}