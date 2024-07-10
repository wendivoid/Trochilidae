use bevy_ecs::prelude::*;
use blueprint::{nodes, ExecutionError, FromGraphValue, GraphValue};

#[derive(Component, Debug, Default, PartialEq, Clone)]
pub struct Elevation(pub f32);

impl<'a> FromGraphValue<'a> for Elevation {
    const NODE_NAME: &'static str = std::any::type_name::<nodes::Cell>();
    const PROPERTY: &'a str = "elevation";

    fn from_graph_value(value: GraphValue) -> Result<Self, ExecutionError> {
        Ok(Elevation(value.to_float()? as f32))
    }
}