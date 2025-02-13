use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_render::{
    mesh::{Indices, Mesh, PrimitiveTopology},
    render_asset::RenderAssetUsages,
};
use bevy_tasks::AsyncComputeTaskPool;
use bevy_utils::hashbrown::HashMap;
use hexx::{ColumnMeshBuilder, Hex, HexLayout, MeshInfo};

use crate::moisture::MoistureBundle;
use crate::{
    moisture::{Moisture, MoistureMeshHandle},
    EntityMap, InsertWorldChunk, WorldSettings,
};

pub fn spawn_moisture(
    mut commands: Commands,
    map: Res<EntityMap>,
    settings: Res<WorldSettings>,
    mut events: EventReader<InsertWorldChunk>,
    data_query: Query<&Moisture>,
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
            .map(|(wrapped, world)| {
                map.inner
                    .get(wrapped)
                    .map(|x| data_query.get(*x).ok().map(|x| (*world, x.0)))
                    .unwrap_or_default()
            })
            .filter_map(|m| m.map(|m| if m.1 > 0.5 { Some(m) } else { None }))
            .flatten()
            .collect();
        let pool = AsyncComputeTaskPool::get();
        let layout = settings.layout();
        let center = descriptor.center;

        let task = pool.spawn(async move { build_moisture_mesh(cells, center, layout) });
        if let Some(mut entity) = commands.get_entity(*entity) {
            entity.with_children(|commands| {
                commands.spawn((MoistureBundle::default(), MoistureMeshHandle { task }));
            });
        }
    }
}

fn build_moisture_mesh(cells: HashMap<Hex, f32>, center: Hex, layout: HexLayout) -> Mesh {
    let mut mesh_info = MeshInfo::default();
    for (cell, level) in cells.iter() {
        let h = ColumnMeshBuilder::new(&layout, 1.0)
            .at(*cell - center)
            .with_offset(bevy_math::Vec3::new(0.0, *level, 0.0))
            .center_aligned()
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
