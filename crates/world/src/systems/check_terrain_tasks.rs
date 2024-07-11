use bevy_asset::Assets;
use bevy_ecs::prelude::*;
use bevy_render::prelude::*;
use bevy_tasks::{block_on, futures_lite::future};

use crate::{components::{Chunk, TerrainMeshHandle}, core::MeshCache};

pub fn check_terrain_tasks(
    mut commands: Commands,
    mut cache: ResMut<MeshCache>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(Entity, &Chunk, &mut TerrainMeshHandle)>
) {
    for (entity, cell, mut handle) in query.iter_mut() {
        if let Some(mesh) = block_on(future::poll_once(&mut handle.task)) {
            let mesh_handle = meshes.add(mesh);
            if handle.cache {
                cache.inner.insert(cell.0, entity);
            }
            if let Some(mut entity_builder) = commands.get_entity(entity) {
                entity_builder
                    .insert(mesh_handle)
                    .remove::<TerrainMeshHandle>();
            }
        }
    }
}