use bevy_utils::HashMap;
use petgraph::Direction;

use crate::{
    graph::{GraphEdge, GraphNode, Node, NodeConstructor, NodeGraph, PropertyCollection},
    nodes,
    value::{FromGraphValue, IntoGraphValue},
    ExecutionError, GraphValue, NodeRegistry,
};

#[derive(Default, Clone)]
pub struct BlueprintData {
    pub graph: NodeGraph,
    pub properties: HashMap<GraphNode, (String, PropertyCollection)>,
}

#[derive(Default)]
pub struct Blueprint {
    pub data: BlueprintData,
    pub registry: NodeRegistry,
}

impl Blueprint {
    pub fn new() -> Blueprint {
        Blueprint::default()
    }

    pub fn with_default() -> Result<Blueprint, ExecutionError> {
        let mut blueprint = Blueprint::default();
        blueprint.init_node::<nodes::Cell>()?;
        blueprint.init_node::<nodes::Math>()?;
        blueprint.init_node::<nodes::Noise>()?;
        blueprint.init_node::<nodes::Coord>()?;
        blueprint.init_node::<nodes::ColorRamp>()?;
        Ok(blueprint)
    }

    pub fn init_node<'a, N: NodeConstructor<'a>>(&mut self) -> Result<(), ExecutionError> {
        Ok(self.registry.init::<N>()?)
    }

    pub fn add_node<'a, N: Node + 'static>(
        &mut self,
        name: &'a str,
        value: N,
    ) -> Result<(), ExecutionError> {
        Ok(self.registry.register(name, value)?)
    }

    pub fn rm_node<'a, N: NodeConstructor<'a>>(&mut self) {
        self.registry.remove::<N>()
    }

    pub fn insert_attribute<'a, V: IntoGraphValue>(
        &mut self,
        node: GraphNode,
        name: &'a str,
        value: V,
    ) {
        if let Some((node_name, mut attrs)) = self.data.properties.remove(&node) {
            attrs.inner.insert(name.to_string(), value.into_value());
            self.data.properties.insert(node, (node_name, attrs));
        }
    }

    pub fn add_graph_node<'a>(
        &mut self,
        ty: &'a str,
        node: GraphNode,
    ) -> Result<GraphNode, ExecutionError> {
        let properties = Default::default();
        let node_runner = self
            .registry
            .get_mut(ty)
            .map_or_else(|| Err(node), |x| Ok(x))?;
        node_runner.initialize(node, &properties)?;
        self.data
            .properties
            .insert(node, (ty.to_string(), properties));
        Ok(self.data.graph.add_node(node))
    }

    pub fn rm_graph_node(&mut self, node: GraphNode) -> bool {
        self.data.properties.remove(&node);
        self.data.graph.remove_node(node)
    }

    pub fn add_graph_edge(
        &mut self,
        node1: GraphNode,
        node2: GraphNode,
        edge: GraphEdge,
    ) -> Option<GraphEdge> {
        self.data.graph.add_edge(node1, node2, edge)
    }

    pub fn rm_graph_edge(&mut self, node1: GraphNode, node2: GraphNode) -> Option<GraphEdge> {
        self.data.graph.remove_edge(node1, node2)
    }

    pub fn get_value_from_input<'a>(
        &self,
        node: GraphNode,
        property: &'a str,
    ) -> Result<Option<GraphValue>, ExecutionError> {
        for (node2, _, edge) in self.data.graph.edges_directed(node, Direction::Incoming) {
            if let Some(prop) = edge.inputs.iter().position(|x| property == *x) {
                return self.get_value_from_output(node2, &edge.outputs[prop]);
            }
        }
        Ok(None)
    }

    pub fn get_value_from_output<'a>(
        &self,
        node: GraphNode,
        property: &'a str,
    ) -> Result<Option<GraphValue>, ExecutionError> {
        let mut inputs = HashMap::new();
        for (node2, _, edge) in self.data.graph.edges_directed(node, Direction::Incoming) {
            for (index, name) in edge.inputs.iter().enumerate() {
                if let Ok(Some(value)) = self.get_value_from_output(node2, &edge.outputs[index]) {
                    inputs.insert(name.to_string(), value);
                }
            }
        }
        if let Some((node_type, node_data)) = self.data.properties.get(&node) {
            let node_runner = self.registry.get(node_type).unwrap();
            return Ok(node_runner.get_property_value(node, property, &inputs, node_data)?);
        }
        Ok(None)
    }

    pub fn retrieve<'a, N: FromGraphValue<'a>>(&mut self) -> Result<N, ExecutionError> {
        for (node_id, (node_name, _)) in self.data.properties.iter() {
            if *node_name == N::NODE_NAME {
                if let Some(value) = self.get_value_from_input(*node_id, N::PROPERTY)? {
                    return N::from_graph_value(value);
                }
            }
        }
        Ok(N::default())
    }

    pub fn retrieve_or_default<'a, N: FromGraphValue<'a>>(&mut self) -> N {
        self.retrieve::<N>().unwrap_or_else(|_| N::default())
    }
}
