use bevy_utils::HashMap;

use crate::{
    graph::{GraphNode, InputCollection, Node, NodeConstructor, PropertyCollection, PropertyType},
    ExecutionError, Value, ValueType,
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
    ) -> Result<Option<Value>, ExecutionError> {
        if property == "value" {
            if let Some(Value::String(op)) = properties.get("operation") {
                let a = if let Some(value) = incoming_properties.get("a") {
                    value.clone()
                } else if let Some(value) = properties.get("a") {
                    value.clone()
                } else {
                    return Err(ExecutionError::MissingProperty {
                        node: node,
                        property: "a".into(),
                    });
                };
                let b = if let Some(value) = incoming_properties.get("b") {
                    value.clone()
                } else if let Some(value) = properties.get("b") {
                    value.clone()
                } else {
                    return Err(ExecutionError::MissingProperty {
                        node: node,
                        property: "b".into(),
                    });
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
        map.insert("value".into(), PropertyType::Output(ValueType::Any));
        map.insert("a".into(), PropertyType::Input(ValueType::Any));
        map.insert("b".into(), PropertyType::Input(ValueType::Any));
        map
    }
}
