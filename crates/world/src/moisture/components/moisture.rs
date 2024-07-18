use bevy_ecs::prelude::*;
use blueprint::{nodes, ExecutionError, FromGraphValue, GraphValue};
use derive_more::{Deref, DerefMut};

#[derive(Component, Debug, Default, PartialEq, Clone, Deref, DerefMut)]
pub struct Moisture(pub f32);

impl<'a> FromGraphValue<'a> for Moisture {
    const NODE_NAME: &'static str = std::any::type_name::<nodes::Cell>();
    const PROPERTY: &'a str = "moisture";

    fn from_graph_value(value: GraphValue) -> Result<Self, ExecutionError> {
        Ok(Moisture(value.to_float()? as f32))
    }
}
