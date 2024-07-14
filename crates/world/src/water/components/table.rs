use bevy_ecs::prelude::*;
use derive_more::Deref;
use blueprint::{nodes, ExecutionError, FromGraphValue, GraphValue};

#[derive(Component, Debug, Default, PartialEq, Clone, Deref)]
pub struct WaterTable(pub f32);

impl<'a> FromGraphValue<'a> for WaterTable {
    const NODE_NAME: &'static str = std::any::type_name::<nodes::Cell>();
    const PROPERTY: &'a str = "water_table";

    fn from_graph_value(value: GraphValue) -> Result<Self, ExecutionError> {
        Ok(WaterTable(value.to_float()? as f32))
    }
}