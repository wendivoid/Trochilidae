use bevy_math::prelude::*;
use hexx::Hex;

use crate::WorldSettings;

pub struct MeshDescriptor {
    pub chunk: Hex,
    pub center: Hex,
    pub position: Vec2,
    pub cachable: bool,
}

impl MeshDescriptor {
    pub fn new(settings: &WorldSettings, chunk: Hex, total_cells: usize) -> MeshDescriptor {
        let center = chunk.to_higher_res(settings.chunk_radius);
        let position = settings.layout().hex_to_world_pos(center);
        MeshDescriptor {
            chunk,
            center,
            position,
            cachable: total_cells == settings.chunk_hex_count(),
        }
    }

    pub fn world_position(&self) -> Vec3 {
        Vec3::new(self.position.x, 0.0, self.position.y)
    }
}
