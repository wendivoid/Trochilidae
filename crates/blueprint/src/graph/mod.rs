mod graph_edge;
mod graph_node;
mod node;

use bevy_utils::HashMap;
use petgraph::graphmap::DiGraphMap;

pub use self::graph_edge::GraphEdge;
pub use self::graph_node::GraphNode;
pub use self::node::{Node, NodeConstructor, PropertyType};

pub type NodeGraph = DiGraphMap<GraphNode, GraphEdge>;

pub type PropertyCollection = HashMap<String, crate::Value>;

pub type InputCollection<'a> = &'a HashMap<String, crate::Value>;
