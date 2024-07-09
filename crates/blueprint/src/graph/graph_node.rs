use uuid::Uuid;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Hash, Eq, Ord)]
pub struct GraphNode(Uuid);

impl Default for GraphNode {
    fn default() -> GraphNode {
        GraphNode(Uuid::new_v4())
    }
}
