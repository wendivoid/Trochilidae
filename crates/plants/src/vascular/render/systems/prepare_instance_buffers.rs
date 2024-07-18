use bevy_ecs::prelude::*;
use bevy_render::{
    render_resource::{BufferInitDescriptor, BufferUsages},
    renderer::RenderDevice,
};

use crate::vascular::render::{data::VascularInstanceMap, VascularBuffer};

pub fn prepare_instance_buffers(
    mut commands: Commands,
    query: Query<(Entity, &VascularInstanceMap)>,
    render_device: Res<RenderDevice>,
) {
    for (entity, instance_data) in &query {
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("instance data buffer"),
            contents: bytemuck::cast_slice(instance_data.0.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        commands.entity(entity).insert(VascularBuffer {
            buffer,
            length: instance_data.len(),
        });
    }
}
