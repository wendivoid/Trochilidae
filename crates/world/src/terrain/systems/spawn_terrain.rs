use bevy_color::{Color, ColorToComponents};
use bevy_ecs::prelude::*;
use bevy_render::{
    mesh::{Indices, Mesh, PrimitiveTopology},
    render_asset::RenderAssetUsages,
};
use bevy_tasks::AsyncComputeTaskPool;
use bevy_utils::hashbrown::HashMap;
use hexx::{ColumnMeshBuilder, Hex, HexLayout};

use crate::{
    terrain::{CellColor, Elevation, TerrainMeshHandle},
    EntityMap, InsertWorldChunk, WorldSettings,
};

pub fn spawn_terrain(
    mut commands: Commands,
    map: Res<EntityMap>,
    settings: Res<WorldSettings>,
    mut events: EventReader<InsertWorldChunk>,
    data_query: Query<(&Elevation, &CellColor)>,
) {
    for InsertWorldChunk { entity, descriptor, cells } in events.read() {
        let entities: HashMap<Hex, (f32, Color)> = cells
            .clone()
            .iter()
            .map(|(wrapped, _)| {
                map.inner
                    .get(wrapped)
                    .map(|x| data_query.get(*x).ok().map(|(y, z)| (*wrapped, (y.0, z.0))))
                    .unwrap_or_default()
            })
            .flatten()
            .collect();
        let pool = AsyncComputeTaskPool::get();
        let layout = settings.layout();
        let center = descriptor.center;
        let cache = cells.len() == settings.chunk_bounds().hex_count();
        let cells = cells.clone();
        let task = pool.spawn(async move {
            build_terrain_mesh(
                center,
                cells,
                entities,
                layout,
            )
        });
        commands.entity(*entity).insert(TerrainMeshHandle { task, cache });
    }
}

fn build_terrain_mesh(
    chunk_center: Hex,
    cells: Vec<(Hex, Hex)>,
    entities: HashMap<Hex, (f32, Color)>,
    layout: HexLayout,
) -> Mesh {
    let mut mesh_info = hexx::MeshInfo::default();
    let mut colors: Vec<[f32; 4]> = vec![];
    for (wrapped, cell) in cells {
        let (height, color) = &entities[&wrapped];
        let h = ColumnMeshBuilder::new(&layout, 10.0)
            .at(cell - chunk_center)
            .with_offset(bevy_math::Vec3::new(0.0, *height, 0.0))
            .center_aligned()
            .build();
        colors.extend(vec![color.to_srgba().to_f32_array(); h.vertices.len()]);
        mesh_info.merge_with(h);
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}
