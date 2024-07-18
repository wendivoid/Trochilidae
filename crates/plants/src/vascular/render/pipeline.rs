use std::num::NonZero;

use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_pbr::{MeshPipeline, MeshPipelineKey};
use bevy_render::{
    mesh::{Mesh, MeshVertexBufferLayoutRef},
    render_resource::{
        BindGroupLayout, BindGroupLayoutEntry, BindingType, BufferBindingType,
        RenderPipelineDescriptor, Shader, ShaderStages, SpecializedMeshPipeline,
        SpecializedMeshPipelineError, VertexAttribute, VertexBufferLayout, VertexFormat,
        VertexStepMode,
    },
    renderer::RenderDevice,
};

use crate::vascular::mesh::ATTRIBUTE_GENERATION;

use super::VascularData;
#[derive(Resource, Clone)]
pub struct VascularPipeline {
    mesh_pipeline: MeshPipeline,
    fragment_shader: Handle<Shader>,
    vertex_shader: Handle<Shader>,
    _vertex_output_shader: Handle<Shader>,
    pub(crate) instance_index_bind_group_layout: BindGroupLayout,
}

impl FromWorld for VascularPipeline {
    fn from_world(world: &mut World) -> Self {
        let mesh_pipeline = world.resource::<MeshPipeline>();
        let render_device = world.resource::<RenderDevice>().clone();
        let instance_index_bind_group_layout = render_device.create_bind_group_layout(
            Some("instance index bind group layout"),
            &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: NonZero::new(16),
                },
                count: None,
            }],
        );

        VascularPipeline {
            mesh_pipeline: mesh_pipeline.clone(),
            instance_index_bind_group_layout,
            _vertex_output_shader: world.load_asset(super::VERTEX_OUTPUT_SHADER_ASSET_PATH),
            vertex_shader: world.load_asset(super::VERTEX_SHADER_ASSET_PATH),
            fragment_shader: world.load_asset(super::FRAGMENT_SHADER_ASSET_PATH),
        }
    }
}

impl SpecializedMeshPipeline for VascularPipeline {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;

        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(2),
            ATTRIBUTE_GENERATION.at_shader_location(5),
        ])?;

        let instance_layout = VertexBufferLayout {
            array_stride: std::mem::size_of::<VascularData>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 3,
                },
                VertexAttribute {
                    format: VertexFormat::Float32,
                    offset: VertexFormat::Float32x4.size(),
                    shader_location: 4,
                },
            ],
        };
        descriptor.vertex.shader = self.vertex_shader.clone();
        descriptor.fragment.as_mut().unwrap().shader = self.fragment_shader.clone();
        descriptor.vertex.buffers = vec![vertex_layout, instance_layout];
        descriptor
            .layout
            .push(self.instance_index_bind_group_layout.clone());
        Ok(descriptor)
    }
}
