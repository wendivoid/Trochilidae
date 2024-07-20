use bevy_ecs::prelude::*;

use bevy_render::render_resource::{BindGroup, Buffer};

#[derive(Component)]
pub struct VascularBuffer {
    pub buffer: Buffer,
    pub length: usize,
}

#[derive(Component)]
pub struct IndexBindgroup {
    pub bind_group: BindGroup,
}
