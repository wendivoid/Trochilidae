use bevy_color::{Color, ColorToComponents};
use bevy_render::{mesh::{Mesh, Indices, PrimitiveTopology}, render_asset::RenderAssetUsages};
use bevy_utils::HashMap;
use hexx::{ColumnMeshBuilder, Hex};

use crate::WorldSettings;

pub struct ChunkMeshBuilder {
    bounds: hexx::HexBounds,
    layout: hexx::HexLayout,
    chunk_center: Hex,
    entities: bevy_utils::HashMap<Hex, (f32, Color)>,
}

impl ChunkMeshBuilder {
    pub fn new(
        center: Hex,
        entities: HashMap<Hex, (f32, Color)>,
        settings: &WorldSettings,
    ) -> ChunkMeshBuilder {
        ChunkMeshBuilder {
            bounds: settings.bounds(),
            layout: settings.layout(),
            chunk_center: center,
            entities,
        }
    }

    pub fn build(&self, cells: impl Iterator<Item = Hex>) -> Mesh {
        let mut mesh_info = hexx::MeshInfo::default();
        let mut colors: Vec<[f32; 4]> = vec![];
        for cell in cells {
            let (height, color) = self.entities[&self.bounds.wrap(cell)];
            let h = ColumnMeshBuilder::new(&self.layout, height)
                .at(cell - self.chunk_center)
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
    }
}
