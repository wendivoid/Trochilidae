use bevy_ecs::prelude::*;
use blueprint::{nodes, ExecutionError, GraphValue, Value};

#[derive(Component, Debug, Default, PartialEq, Clone)]
pub struct Elevation(pub f32);

impl<'a> GraphValue<'a> for Elevation {
    const NODE_NAME: &'static str = std::any::type_name::<nodes::Cell>();
    const PROPERTY: &'a str = "elevation";

    fn from_graph_value(value: Value) -> Result<Self, ExecutionError> {
        Ok(Elevation(value.to_float()? as f32))
    }
}