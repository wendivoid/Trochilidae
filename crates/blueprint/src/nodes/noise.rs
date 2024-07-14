use std::rc::Rc;

use bevy_utils::HashMap;
use noise::NoiseFn;

use crate::{
    graph::{GraphNode, Node, NodeConstructor, PropertyType},
    value::GraphValueType,
    ExecutionError, GraphValue,
};

pub enum NoiseGenerator {
    Worley(noise::Worley),
    Perlin(noise::Perlin),
}

pub struct Noise {
    inner: HashMap<GraphNode, NoiseGenerator>,
}

impl<'a> NodeConstructor<'a> for Noise {
    type Node = Self;

    fn construct() -> Self::Node {
        Noise {
            inner: Default::default(),
        }
    }
}

impl Node for Noise {
    fn initialize<'a>(
        &mut self,
        node: crate::graph::GraphNode,
        attrs: &'a crate::graph::PropertyCollection,
    ) -> Result<(), ExecutionError> {
        let seed = if let Some(value) = attrs.inner.get("seed") {
            value.clone().to_int()? as u32
        } else {
            rand::random()
        };
        if let Some(value) = attrs.inner.get("noise_function") {
            match &value.clone().to_string()?[..] {
                "perlin" => {
                    self.inner
                        .insert(node, NoiseGenerator::Perlin(noise::Perlin::new(seed)));
                }
                "worley" => {
                    self.inner
                        .insert(node, NoiseGenerator::Worley(noise::Worley::new(seed)));
                }
                _ => unreachable!(),
            }
        } else {
            self.inner
                .insert(node, NoiseGenerator::Perlin(noise::Perlin::new(seed)));
        }
        Ok(())
    }

    fn available_properties<'a>(&self) -> HashMap<String, PropertyType> {
        let mut map = HashMap::new();
        map.insert("value".into(), PropertyType::Output(GraphValueType::Any));
        map.insert(
            "uv".into(),
            PropertyType::Input(GraphValueType::Vec2(Rc::new(GraphValueType::Float))),
        );
        map.insert(
            "noise_function".into(),
            PropertyType::Stateful(GraphValueType::String),
        );
        map.insert("seed".into(), PropertyType::Stateful(GraphValueType::Int));
        map
    }

    fn get_property_value<'a>(
        &self,
        node: crate::graph::GraphNode,
        property: &'a str,
        inputs: crate::graph::InputCollection,
        _: &'a crate::graph::PropertyCollection,
    ) -> Result<Option<crate::GraphValue>, crate::ExecutionError> {
        if property == "value" {
            if let Some(value) = inputs.get("uv") {
                match value
                    .clone()
                    .to_vec2_value(&std::rc::Rc::new(GraphValueType::Double))?
                {
                    GraphValue::Vec2(a, b) => {
                        let noise = self.inner.get(&node).unwrap();

                        let value = match noise {
                            NoiseGenerator::Perlin(perlin) => {
                                perlin.get([a.to_double()?, b.to_double()?])
                            }
                            NoiseGenerator::Worley(worley) => {
                                worley.get([a.to_double()?, b.to_double()?])
                            }
                        };
                        return Ok(Some(GraphValue::Double(value)));
                    }
                    _ => unreachable!(),
                }
            }
        }
        Ok(None)
    }
}
