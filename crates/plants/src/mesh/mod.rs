mod config;
mod data;
mod renderer;
mod state;

pub use self::config::MeshRenderConfig;
pub use self::renderer::MeshRenderer;
pub use self::state::MeshRenderState;

use bevy_render::{mesh::MeshVertexAttribute, render_resource::VertexFormat};

pub const ATTRIBUTE_GENERATION: MeshVertexAttribute = MeshVertexAttribute::new(
    "ATTRIBUTE_GENERATION",
    334022684658501,
    VertexFormat::Float32,
);
