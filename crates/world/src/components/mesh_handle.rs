use bevy_tasks::Task;
use bevy_ecs::prelude::*;
use bevy_render::prelude::*;

#[derive(Component)]
pub struct MeshHandle {
    pub(crate) cache: bool,
    pub(crate) task: Task<Mesh>
}

impl MeshHandle {
    pub fn new(task: Task<Mesh>, cache: bool) -> MeshHandle {
        MeshHandle { task, cache }
    }
}