use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_math::prelude::*;
use bevy_math::Vec3Swizzles;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_pbr::StandardMaterial;
use bevy_tasks::AsyncComputeTaskPool;
use bevy_transform::prelude::*;
use bevy_utils::HashMap;
use hexx::Hex;

use crate::components::MeshHandle;
use crate::map::HexState;
use crate::mesh::ChunkMeshBuilder;
use crate::mesh::ChunkQueryData;
use crate::{
    bundles::ChunkDisplayBundle,
    components::{Canvas, Observer},
    HexMap, WorldSettings,
};

pub fn update_map(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut map: ResMut<HexMap>,
    settings: Res<WorldSettings>,
    mut state: ResMut<HexState>,
    mut observer: Query<&mut PanOrbitCamera, (With<Observer>, Changed<GlobalTransform>)>,
    canvas: Query<(Entity, Option<&Children>), With<Canvas>>,
    chunk_query_data: ChunkQueryData,
) {
    for mut observer_camera in observer.iter_mut() {
        let wrapped_observer_hex = reposition_observer(&settings, &mut observer_camera);
        let wrapped_observer_pos = observer_camera.focus.xz();
        let observer_chunk = wrapped_observer_hex.to_lower_res(settings.chunk_radius);
        if Some(observer_chunk) != state.active {
            state.active = Some(observer_chunk);
            let pool = AsyncComputeTaskPool::get();
            let mut allowed_entities = HashMap::new();
            for (chunk, cells) in settings.visible_chunks(wrapped_observer_pos) {
                let center = chunk.to_higher_res(settings.chunk_radius);
                let chunk_pos = settings.layout().hex_to_world_pos(center);
                if map.cache.contains_key(&chunk) {
                     commands.entity(map.cache[&chunk]).insert(Transform::from_xyz(chunk_pos.x, 0.0, chunk_pos.y));
                    allowed_entities.insert(map.cache[&chunk].clone(), chunk);
                } else {
                    let entities = chunk_query_data.data(&map, cells.iter());
                    let builder = ChunkMeshBuilder::new(center, entities, &settings);
                    let cache = cells.len() == hexx::shapes::hexagon(Hex::ZERO, settings.chunk_radius).count();
                    let task = pool.spawn(async move {
                        builder.build(cells.into_iter())
                    });
                    let id = commands
                        .spawn(ChunkDisplayBundle::new(chunk, chunk_pos))
                        .insert(MeshHandle { task, cache })
                        .insert(materials.add(StandardMaterial::default()))
                        .set_parent(canvas.single().0)
                        .id();
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
            for (key, value) in map.cache.clone().iter() {
                if allowed_entities.contains_key(value) {
                    map.cache.remove(key);
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
    observer.focus = Vec3::new(
        wrapped_position.x,
        observer.focus.y,
        wrapped_position.y,
    );
    wrapped_observer_hex
}
