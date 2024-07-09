use bevy_utils::HashMap;

use crate::{
    graph::{GraphNode, InputCollection, PropertyCollection},
    ExecutionError, Value, ValueType,
};

pub enum PropertyType {
    Input(ValueType),
    Output(ValueType),
    Stateful(ValueType)
}

pub trait Node {
    fn initialize<'a>(
        &mut self,
        _: GraphNode,
        _: &'a PropertyCollection,
    ) -> Result<(), ExecutionError> {
        Ok(())
    }

    fn available_properties<'a>(&self) -> HashMap<String, PropertyType>;

    fn get_property_value<'a>(
        &self,
        node: GraphNode,
        property: &'a str,
        incoming_properties: InputCollection,
        properties: &'a PropertyCollection,
    ) -> Result<Option<Value>, ExecutionError>;
}

pub trait NodeConstructor<'a> {
    type Node: Node + 'static;
    const NAME: &'a str = std::any::type_name::<Self>();

    fn construct() -> Self::Node;
}
