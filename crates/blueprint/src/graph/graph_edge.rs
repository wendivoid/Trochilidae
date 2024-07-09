#[derive(Debug, PartialEq, Clone)]
pub struct GraphEdge {
    pub outputs: Vec<String>,
    pub inputs: Vec<String>,
}
