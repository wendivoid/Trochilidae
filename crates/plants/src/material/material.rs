use bevy_pbr::{prelude::*, MaterialPipeline, MaterialPipelineKey};
use bevy_asset::prelude::*;
use bevy_reflect::prelude::*;
use bevy_render::{mesh::{MeshVertexBufferLayout, Mesh}, render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError}};

const SHADER_ASSET_PATH: &str = "shaders/plant.wgsl";
const PREPASS_SHADER_ASSET_PATH: &str = "shaders/prepass.wgsl";

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone, Default)]
pub struct PlantMaterial {
    #[uniform(0)]
    pub time_scale: f32,
}

impl Material for PlantMaterial {
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn prepass_fragment_shader() -> ShaderRef {
        PREPASS_SHADER_ASSET_PATH.into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        PREPASS_SHADER_ASSET_PATH.into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(7),
            crate::mesh::ATTRIBUTE_GENERATION.at_shader_location(8),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}