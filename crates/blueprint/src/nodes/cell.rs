use bevy_utils::HashMap;

use crate::{
    graph::{GraphNode, InputCollection, Node, NodeConstructor, PropertyCollection, PropertyType},
    ExecutionError, Value, ValueType,
};

pub struct Cell;

impl<'a> NodeConstructor<'a> for Cell {
    type Node = Self;

    fn construct() -> Self::Node {
        Cell
    }
}

impl Node for Cell {
    fn get_property_value<'a>(
        &self,
        _: GraphNode,
        _: &'a str,
        _: InputCollection,
        _: &'a PropertyCollection,
    ) -> Result<Option<Value>, ExecutionError> {
        unreachable!()
    }
    
    fn available_properties<'a>(&self) -> HashMap<String, PropertyType> {
        let mut map = HashMap::new();
        map.insert("elevation".into(), PropertyType::Input(ValueType::Float));
        map
    }
}