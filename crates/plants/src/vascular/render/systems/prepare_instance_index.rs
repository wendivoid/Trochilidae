use bevy_core_pipeline::core_3d::Transparent3d;
use bevy_ecs::prelude::*;
use bevy_render::{
    render_phase::ViewSortedRenderPhases,
    render_resource::{
        BindGroupEntries, BindingResource, BufferBinding, BufferInitDescriptor, BufferUsages,
    },
    renderer::RenderDevice,
};

use crate::vascular::render::{IndexBindgroup, VascularInstanceData, VascularPipeline};

pub fn prepare_instance_index(
    query: Query<Entity, With<VascularInstanceData>>,
    mut commands: Commands,
    phases: Res<ViewSortedRenderPhases<Transparent3d>>,
    pipeline: Res<VascularPipeline>,
    render_device: Res<RenderDevice>,
) {
    for entity in &query {
        let Some(item) = phases
            .iter()
            .flat_map(|(_, phase)| &phase.items)
            .find(|item| item.entity == entity)
        else {
            continue;
        };
        let index_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("instance index buffer"),
            contents: bytemuck::cast_slice(&[item.batch_range.start, 0, 0, 0]),
            usage: BufferUsages::VERTEX | BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let layout = &pipeline.instance_index_bind_group_layout;
        let bind_group = render_device.create_bind_group(
            "instance index bindgroup",
            layout,
            &BindGroupEntries::single(BindingResource::Buffer(BufferBinding {
                buffer: &index_buffer,
                offset: 0,
                size: None,
            })),
        );

        commands
            .entity(entity)
            .insert(IndexBindgroup { bind_group });
    }
}
