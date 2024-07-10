use std::rc::Rc;

use bevy_utils::HashMap;

use crate::{
    graph::{GraphNode, InputCollection, Node, NodeConstructor, PropertyCollection, PropertyType}, value::IntoGraphValue, ExecutionError, GraphValue, GraphValueType
};

pub struct Coord(pub hexx::Hex);

impl<'a> NodeConstructor<'a> for Coord {
    type Node = Self;

    fn construct() -> Self::Node {
        Coord(Default::default())
    }
}

impl Node for Coord {
    fn get_property_value<'a>(
        &self,
        node: GraphNode,
        property: &'a str,
        _: InputCollection,
        _: &'a PropertyCollection,
    ) -> Result<Option<GraphValue>, ExecutionError> {
        match property {
            "q" => Ok(Some(self.0.x().into_value())),
            "r" => Ok(Some(self.0.y().into_value())),
            "axial" => Ok(Some(GraphValue::vec2(
                self.0.x,
                self.0.y,
            ))),
            _ => Err(crate::ExecutionError::InvalidProperty {
                node: node,
                property: property.into(),
            }),
        }
    }

    fn available_properties<'a>(&self) -> HashMap<String, PropertyType> {
        let mut map = HashMap::new();
        map.insert("q".into(), PropertyType::Output(GraphValueType::Int));
        map.insert("r".into(), PropertyType::Output(GraphValueType::Int));
        map.insert("s".into(), PropertyType::Output(GraphValueType::Int));
        map.insert("axial".into(), PropertyType::Output(GraphValueType::Vec2(Rc::new(GraphValueType::Int))));
        map.insert("cube".into(), PropertyType::Output(GraphValueType::Vec3(Rc::new(GraphValueType::Int))));
        map
    }
}
