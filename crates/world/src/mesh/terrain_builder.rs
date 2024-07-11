use bevy_color::{Color, ColorToComponents};
use bevy_render::{mesh::{Mesh, Indices, PrimitiveTopology}, render_asset::RenderAssetUsages};
use bevy_utils::HashMap;
use hexx::{ColumnMeshBuilder, Hex, HexLayout};

use crate::core::WorldSettings;

pub struct TerrainMeshBuilder {
    layout: HexLayout,
    chunk_center: Hex,
    entities: HashMap<Hex, (f32, Color, f32, f32)>,
}

impl TerrainMeshBuilder {
    pub fn new(
        center: Hex,
        entities: HashMap<Hex, (f32, Color, f32, f32)>,
        settings: &WorldSettings,
    ) -> TerrainMeshBuilder {
        TerrainMeshBuilder {
            layout: settings.layout(),
            chunk_center: center,
            entities,
        }
    }

    pub fn build(&self, cells: impl Iterator<Item = (Hex, Hex)>) -> Mesh {
        let mut mesh_info = hexx::MeshInfo::default();
        let mut colors: Vec<[f32; 4]> = vec![];
        for (wrapped, cell) in cells {
            let (height, color, _, _) = self.entities[&wrapped];
            let h = ColumnMeshBuilder::new(&self.layout, 5.0)
                .at(cell - self.chunk_center)
                .with_offset(bevy_math::Vec3::new(0.0, height, 0.0))
                .without_bottom_face()
                .center_aligned()
                .build();
            colors.extend(vec![color.to_srgba().to_f32_array(); h.vertices.len()]);
            mesh_info.merge_with(h);
        }
        Self::hex_mesh(mesh_info, colors)
    }

    fn hex_mesh(mesh_info: hexx::MeshInfo, colors: Vec<[f32; 4]>) -> Mesh {
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
        .with_inserted_indices(Indices::U16(mesh_info.indices))
        .with_duplicated_vertices()
        .with_computed_flat_normals()
    }
}
