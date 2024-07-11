use bevy_asset::Assets;
use bevy_ecs::prelude::*;
use bevy_pbr::prelude::*;
use bevy_render::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_tasks::{block_on, futures_lite::future};
use bevy_transform::components::Transform;

use crate::{components::{MoistureEntity, MoistureMeshHandle}, MOISTURE_MATERIAL};

pub fn check_moisture_tasks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(Entity, Option<&MoistureEntity>, &mut MoistureMeshHandle)>
) {
    for (entity, water, mut handle) in query.iter_mut() {
        if let Some(mesh) = block_on(future::poll_once(&mut handle.task)) {
            if let Some(entity) = water {
                commands.entity(entity.0).insert(meshes.add(mesh));
            } else {
                commands.entity(entity).with_children(|commands| {
                    commands.spawn(MaterialMeshBundle {
                        mesh: meshes.add(mesh),
                        transform: Transform::from_xyz(0.0, 20.0, 0.0),
                        material: MOISTURE_MATERIAL,
                        ..Default::default()
                    });
                });
            }
            commands.entity(entity).remove::<MoistureMeshHandle>();
        }
    }
}