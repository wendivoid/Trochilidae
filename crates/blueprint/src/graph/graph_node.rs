use uuid::Uuid;
use derive_more::Deref;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Hash, Eq, Ord, Deref)]
pub struct GraphNode(Uuid);

impl Default for GraphNode {
    fn default() -> GraphNode {
        GraphNode(Uuid::new_v4())
    }
}
