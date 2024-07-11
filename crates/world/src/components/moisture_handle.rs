use bevy_tasks::Task;
use bevy_ecs::prelude::*;
use bevy_render::prelude::*;

#[derive(Component)]
pub struct MoistureMeshHandle {
    pub(crate) task: Task<Mesh>
}

impl MoistureMeshHandle {
    pub fn new(task: Task<Mesh>) -> MoistureMeshHandle {
        MoistureMeshHandle { task }
    }
}

#[derive(Component)]
pub struct MoistureEntity(pub Entity);