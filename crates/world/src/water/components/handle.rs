use bevy_ecs::prelude::*;
use bevy_render::prelude::*;
use bevy_tasks::Task;

#[derive(Component)]
pub struct WaterMeshHandle {
    pub(crate) task: Task<Mesh>,
}

impl WaterMeshHandle {
    pub fn new(task: Task<Mesh>) -> WaterMeshHandle {
        WaterMeshHandle { task }
    }
}
