use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_math::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_render::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy_render::render_asset::RenderAssetUsages;
use bevy_tasks::prelude::*;
use bevy_transform::prelude::*;
use bevy_utils::HashMap;
use hexx::{ColumnMeshBuilder, Hex, HexLayout, MeshInfo, UVOptions};

use crate::mesh::{ChunkQueryData, MeshDescriptor};
use crate::utils::CommandExt;
use crate::EntityCache;
use crate::{
    components::{Canvas, Chunk, Observer},
    core::{MeshCache, WorldOrigin, WorldSettings},
};

pub fn update_mesh_tasks(
    mut commands: Commands,
    map: Res<EntityCache>,
    settings: Res<WorldSettings>,
    mut state: ResMut<WorldOrigin>,
    mut cache: ResMut<MeshCache>,
    mut observer: Query<&mut PanOrbitCamera, (With<Observer>, Changed<GlobalTransform>)>,
    canvas: Query<(Entity, Option<&Children>), With<Canvas>>,
    chunks: Query<(Entity, &Chunk)>,
    query_data: ChunkQueryData,
) {
    for mut observer_camera in observer.iter_mut() {
        let wrapped_observer_hex = reposition_observer(&settings, &mut observer_camera);
        let wrapped_observer_pos = observer_camera.target_focus.xz();
        let observer_chunk = wrapped_observer_hex.to_lower_res(settings.chunk_radius);
        state.hex = Some(wrapped_observer_hex);
        if Some(observer_chunk) != state.chunk {
            state.chunk = Some(observer_chunk);
            let canvas_entity = canvas.single().0;
            let pool = AsyncComputeTaskPool::get();
            let mut allowed_entities = HashMap::new();
            for (chunk, cells) in settings.visible_chunks(wrapped_observer_pos) {
                let descriptor = MeshDescriptor::new(&settings, chunk, cells.len());
                if cache.inner.contains_key(&chunk) {
                    commands.translate_entity(cache.inner[&chunk], descriptor.world_position());
                    allowed_entities.insert(cache.inner[&chunk], chunk);
                } else {
                    let layout = settings.layout();
                    let layout2 = layout.clone();
                    let radius = settings.world_radius as f32;
                    let rect = hexx::Rect { min: Vec2::new(-radius, -radius), max: Vec2::new(radius, radius) };
                    let (t_entities, w_entities, m_entities) = query_data.data(&*map, cells.iter());
                    let builder = query_data.builder(descriptor.center, &*settings, t_entities);
                    let terrain = pool.spawn(async move { builder.build(cells.into_iter()) });
                    let water = pool.spawn(async move {
                        generate_water_mesh(w_entities, descriptor.center, rect, layout)
                    });
                    let moisture = pool.spawn(async move { generate_cloud_mesh(m_entities, descriptor.center, rect, layout2)});
                    let id =
                        commands.spawn_chunk_display(&descriptor, terrain, water, moisture, canvas_entity);
                    allowed_entities.insert(id, chunk);
                }
            }

            // Despawn any chunks that arent visible anymore.
            let (_, children) = canvas.single();
            if let Some(children) = children {
                for child in children.iter() {
                    if !allowed_entities.contains_key(child) {
                        if let Some(entity) = commands.get_entity(*child) {
                            entity.despawn_recursive();
                        }
                        if let Ok((_, chunk)) = chunks.get(*child) {
                            if cache.inner.contains_key(&chunk.0) {
                                cache.inner.remove(&chunk.0);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn generate_cloud_mesh(
    cells: HashMap<Hex, f32>,
    chunk_center: Hex,
    rect: hexx::Rect,
    layout: HexLayout,
) -> Mesh {
    let mut mesh_info = MeshInfo::default();
    for (cell, level) in cells.iter() {
        let h = ColumnMeshBuilder::new(&layout, 1.0)
            .at(*cell - chunk_center)
            .with_offset(bevy_math::Vec3::new(0.0, *level, 0.0))
            .center_aligned()
            .with_caps_uv_options(UVOptions {
                scale_factor: Vec2::splat(0.2),
                flip: BVec2::TRUE,
                offset: layout.hex_to_world_pos(*cell - chunk_center),
                rect,
            })
            .build();
        mesh_info.merge_with(h);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
    .with_duplicated_vertices()
    .with_computed_flat_normals()
}

fn generate_water_mesh(
    cells: HashMap<Hex, f32>,
    chunk_center: Hex,
    rect: hexx::Rect,
    layout: HexLayout,
) -> Mesh {
    let mut mesh_info = MeshInfo::default();
    for (cell, level) in cells.iter() {
        let h = ColumnMeshBuilder::new(&layout, 0.0)
            .at(*cell - chunk_center)
            .with_offset(bevy_math::Vec3::new(0.0, *level, 0.0))
            .without_bottom_face()
            .center_aligned()
            .with_caps_uv_options(UVOptions {
                scale_factor: Vec2::splat(0.2),
                flip: BVec2::TRUE,
                offset: layout.hex_to_world_pos(*cell - chunk_center),
                rect,
            })
            .build();
        mesh_info.merge_with(h);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
    .with_duplicated_vertices()
    .with_computed_flat_normals()
}

fn reposition_observer(settings: &WorldSettings, observer: &mut PanOrbitCamera) -> Hex {
    let layout = settings.layout();
    let bounds = settings.bounds();
    let observer_pos = observer.target_focus.xz();
    let observer_hex = layout.world_pos_to_hex(observer_pos);
    let wrapped_observer_hex = bounds.wrap(observer_hex);
    let fract_pos = observer_pos - layout.hex_to_world_pos(observer_hex);
    let wrapped_position = fract_pos + layout.hex_to_world_pos(wrapped_observer_hex);
    observer.target_focus = Vec3::new(wrapped_position.x, 0.0, wrapped_position.y);
    wrapped_observer_hex
}
