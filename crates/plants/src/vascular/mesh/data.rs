use bevy_render::{
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use super::ATTRIBUTE_GENERATION;

#[derive(Default)]
pub struct MeshData {
    pub positions: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
    pub generations: Vec<f32>,
    pub indices: Vec<u32>,
}

impl Into<Mesh> for MeshData {
    fn into(self) -> Mesh {
        Mesh::new(PrimitiveTopology::TriangleList, Default::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, self.colors)
            .with_inserted_attribute(ATTRIBUTE_GENERATION, self.generations)
            .with_inserted_indices(Indices::U32(self.indices))
            .with_duplicated_vertices()
            .with_computed_flat_normals()
        //.rotated_by(bevy_math::Quat::from_rotation_x(90f32.to_radians()))
    }
}
