use bevy_utils::HashMap;

use crate::{
    graph::{GraphNode, InputCollection, Node, NodeConstructor, PropertyCollection, PropertyType},
    ExecutionError, GraphValue, GraphValueType,
};

pub struct Math;

impl<'a> NodeConstructor<'a> for Math {
    type Node = Self;

    fn construct() -> Self::Node {
        Math
    }
}

impl Node for Math {
    fn get_property_value<'a>(
        &self,
        node: GraphNode,
        property: &'a str,
        incoming_properties: InputCollection,
        properties: &'a PropertyCollection,
    ) -> Result<Option<GraphValue>, ExecutionError> {
        if property == "value" {
            if let Some(GraphValue::String(op)) = properties.inner.get("operation") {
                let a = if let Some(value) = incoming_properties.get("a") {
                    value.clone()
                } else {
                    properties.cloned(node, "a")?
                };
                let b = if let Some(value) = incoming_properties.get("b") {
                    value.clone()
                } else{
                    properties.cloned(node, "b")?
                };
                let res = match &op[..] {
                    "+" => a.add(b).map(|x| Some(x)),
                    "-" => a.sub(b).map(|x| Some(x)),
                    "*" => a.mul(b).map(|x| Some(x)),
                    _ => unreachable!(),
                };
                return res;
            }
        }
        Ok(None)
    }
    
    fn available_properties<'a>(&self) -> HashMap<String, PropertyType> {
        let mut map = HashMap::new();
        map.insert("value".into(), PropertyType::Output(GraphValueType::Any));
        map.insert("a".into(), PropertyType::Input(GraphValueType::Any));
        map.insert("b".into(), PropertyType::Input(GraphValueType::Any));
        map
    }
}
