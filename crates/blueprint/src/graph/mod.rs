mod graph_edge;
mod graph_node;
mod node;

use bevy_utils::HashMap;
use petgraph::graphmap::DiGraphMap;

use crate::ExecutionError;

pub use self::graph_edge::GraphEdge;
pub use self::graph_node::GraphNode;
pub use self::node::{Node, NodeConstructor, PropertyType};

pub type NodeGraph = DiGraphMap<GraphNode, GraphEdge>;

#[derive(Default, Clone)]
pub struct PropertyCollection {
    pub(crate) inner: HashMap<String, crate::GraphValue>,
}

impl PropertyCollection {
    pub fn get(&self, node: GraphNode, key: &str) -> Result<&crate::GraphValue, ExecutionError> {
        self.inner.get(key).ok_or(ExecutionError::MissingProperty {
            property: key.to_string(),
            node,
        })
    }
    pub fn cloned(&self, node: GraphNode, key: &str) -> Result<crate::GraphValue, ExecutionError> {
        self.inner
            .get(key)
            .cloned()
            .ok_or(ExecutionError::MissingProperty {
                property: key.to_string(),
                node,
            })
    }
}

pub type InputCollection<'a> = &'a HashMap<String, crate::GraphValue>;
