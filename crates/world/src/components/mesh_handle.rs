use bevy_tasks::Task;
use bevy_ecs::prelude::*;
use bevy_render::prelude::*;

#[derive(Component)]
pub struct MeshHandle {
    pub cache: bool,
    pub(crate) task: Task<Mesh>
}