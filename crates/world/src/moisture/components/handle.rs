use bevy_tasks::Task;
use derive_more::Deref;
use bevy_ecs::prelude::*;
use bevy_render::prelude::*;

#[derive(Component, Debug, Deref)]
pub struct MoistureMeshHandle {
    pub(crate) task: Task<Mesh>
}

impl MoistureMeshHandle {
    pub fn new(task: Task<Mesh>) -> MoistureMeshHandle {
        MoistureMeshHandle { task }
    }
}

#[derive(Component, Debug, PartialEq, Clone, Deref)]
pub struct MoistureEntity(pub Entity);