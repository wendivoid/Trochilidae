mod buffer;
mod data;
mod draw;
mod pipeline;
mod plugin;
mod systems;

pub use self::buffer::{IndexBindgroup, VascularBuffer};
pub use self::data::{VascularData, VascularInstanceData};
pub use self::draw::{DrawVascular, DrawVascularInstanced};
pub use self::pipeline::VascularPipeline;
pub use self::plugin::VascularMaterialPlugin;

pub const VERTEX_SHADER_ASSET_PATH: &'static str = "shaders/vascular/vertex.wgsl";
pub const FRAGMENT_SHADER_ASSET_PATH: &'static str = "shaders/vascular/fragment.wgsl";
pub const VERTEX_OUTPUT_SHADER_ASSET_PATH: &'static str = "shaders/vascular/vertex_output.wgsl";
