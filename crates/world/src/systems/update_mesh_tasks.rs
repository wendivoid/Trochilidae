use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_math::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_tasks::prelude::*;
use bevy_transform::prelude::*;
use bevy_utils::HashMap;
use hexx::Hex;

use crate::core::{MeshCache, WorldOrigin};
use crate::mesh::{ChunkQueryData, MeshDescriptor};
use crate::utils::CommandExt;
use crate::EntityCache;
use crate::{
    components::{Canvas, Observer},
    core::WorldSettings,
};

pub fn update_mesh_tasks(
    mut commands: Commands,
    map: Res<EntityCache>,
    settings: Res<WorldSettings>,
    mut state: ResMut<WorldOrigin>,
    mut cache: ResMut<MeshCache>,
    mut observer: Query<&mut PanOrbitCamera, (With<Observer>, Changed<GlobalTransform>)>,
    canvas: Query<(Entity, Option<&Children>), With<Canvas>>,
    query_data: ChunkQueryData,
) {
    for mut observer_camera in observer.iter_mut() {
        let wrapped_observer_hex = reposition_observer(&settings, &mut observer_camera);
        let wrapped_observer_pos = observer_camera.focus.xz();
        let observer_chunk = wrapped_observer_hex.to_lower_res(settings.chunk_radius);
        if Some(observer_chunk) != state.active {
            let canvas_entity = canvas.single().0;
            state.active = Some(observer_chunk);
            let pool = AsyncComputeTaskPool::get();
            let mut allowed_entities = HashMap::new();
            for (chunk, cells) in settings.visible_chunks(wrapped_observer_pos) {
                let descriptor = MeshDescriptor::new(&settings, chunk, cells.len());
                if cache.inner.contains_key(&chunk) {
                    commands.translate_entity(cache.inner[&chunk], descriptor.world_position());
                    allowed_entities.insert(cache.inner[&chunk].clone(), chunk);
                } else {
                    let entities = query_data.data(&*map, cells.iter());
                    let builder = query_data.builder(descriptor.center, &*settings, entities);
                    let task = pool.spawn(async move { builder.build(cells.into_iter()) });
                    let id = commands.spawn_chunk_display(&descriptor, task, canvas_entity);
                    allowed_entities.insert(id, chunk);
                }
            }
            let (_, children) = canvas.single();
            if let Some(children) = children {
                for child in children.iter() {
                    if !allowed_entities.contains_key(child) {
                        if let Some(mut entity) = commands.get_entity(*child) {
                            entity.despawn();
                        }
                    }
                }
            }
            for (key, value) in cache.inner.clone().iter() {
                if allowed_entities.contains_key(value) {
                    cache.inner.remove(key);
                }
            }
        }
    }
}

fn reposition_observer(settings: &WorldSettings, observer: &mut PanOrbitCamera) -> Hex {
    let layout = settings.layout();
    let bounds = settings.bounds();
    let observer_pos = observer.focus.xz();
    let observer_hex = layout.world_pos_to_hex(observer_pos);
    let fract_pos = observer_pos - layout.hex_to_world_pos(observer_hex);
    let wrapped_observer_hex = bounds.wrap(observer_hex);
    let wrapped_position = fract_pos + layout.hex_to_world_pos(wrapped_observer_hex);
    observer.focus = Vec3::new(wrapped_position.x, observer.focus.y, wrapped_position.y);
    wrapped_observer_hex
}
