use bevy_tasks::Task;
use bevy_ecs::prelude::*;
use bevy_render::prelude::*;

#[derive(Component)]
pub struct TerrainMeshHandle {
    pub(crate) cache: bool,
    pub(crate) task: Task<Mesh>
}

impl TerrainMeshHandle {
    pub fn new(task: Task<Mesh>, cache: bool) -> TerrainMeshHandle {
        TerrainMeshHandle { task, cache }
    }
}