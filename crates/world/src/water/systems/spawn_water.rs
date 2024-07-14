use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_math::prelude::*;
use bevy_render::{
    mesh::{Indices, Mesh, PrimitiveTopology},
    render_asset::RenderAssetUsages,
};
use bevy_tasks::AsyncComputeTaskPool;
use bevy_utils::hashbrown::HashMap;
use hexx::{ColumnMeshBuilder, Hex, HexLayout, MeshInfo, UVOptions};

use crate::water::WaterBundle;
use crate::{
    water::{WaterMeshHandle, WaterTable},
    EntityMap, InsertWorldChunk, WorldSettings,
};

pub fn spawn_water(
    mut commands: Commands,
    map: Res<EntityMap>,
    settings: Res<WorldSettings>,
    mut events: EventReader<InsertWorldChunk>,
    data_query: Query<&WaterTable>,
) {
    for InsertWorldChunk {
        entity,
        descriptor,
        cells,
    } in events.read()
    {
        let cells: HashMap<Hex, f32> = cells
            .clone()
            .iter()
            .map(|(wrapped, _)| {
                map.inner
                    .get(wrapped)
                    .map(|x| data_query.get(*x).ok().map(|x| (*wrapped, **x)))
                    .unwrap_or_default()
            })
            .flatten()
            .collect();
        let pool = AsyncComputeTaskPool::get();
        let layout = settings.layout();
        let center = descriptor.center;
        let radius = settings.world_radius as f32;
        let rect = hexx::Rect {
            min: Vec2::new(-radius, -radius),
            max: Vec2::new(radius, radius),
        };
        let task = pool.spawn(async move { build_water_mesh(cells, center, rect, layout) });
        if let Some(mut entity) = commands.get_entity(*entity) {
            entity.with_children(|commands| {
                commands.spawn((WaterBundle::default(), WaterMeshHandle { task }));
            });
        }
    }
}

fn build_water_mesh(
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
