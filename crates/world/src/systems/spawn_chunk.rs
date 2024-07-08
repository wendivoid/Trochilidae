use bevy_asset::Assets;
use bevy_ecs::prelude::*;
use bevy_render::prelude::*;
use bevy_tasks::{block_on, futures_lite::future};

use crate::{components::{Chunk, MeshHandle}, HexMap};

pub fn spawn_chunk(
    mut commands: Commands,
    mut map: ResMut<HexMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(Entity, &Chunk, &mut MeshHandle)>
) {
    for (entity, cell, mut handle) in query.iter_mut() {
        if let Some(mesh) = block_on(future::poll_once(&mut handle.task)) {
            let mesh_handle = meshes.add(mesh);
            if handle.cache {
                map.cache.insert(cell.0, entity);
            }
            commands.entity(entity)
                .insert(mesh_handle)
                .remove::<MeshHandle>();
        }
    }
}