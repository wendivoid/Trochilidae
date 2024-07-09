use bevy_ecs::prelude::*;
use bevy_color::prelude::*;
use blueprint::{nodes, ExecutionError, GraphValue, Value};

#[derive(Component, Default, PartialEq, Clone)]
pub struct CellColor(pub Color);

impl<'a> GraphValue<'a> for CellColor {
    const NODE_NAME: &'static str = std::any::type_name::<nodes::Cell>();
    const PROPERTY: &'a str = "color";

    fn from_graph_value(value: Value) -> Result<Self, ExecutionError> {
        if let Value::Vec4(a, b, c, d) = value {
            return Ok(CellColor(Color::srgba(
                a.to_float()? as f32,
                b.to_float()? as f32,
                c.to_float()? as f32,
                d.to_float()? as f32,
            )));
        }
        Ok(CellColor(Color::default()))
    }
}
