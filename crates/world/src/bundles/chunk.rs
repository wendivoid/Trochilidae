use bevy_math::Vec2;
use bevy_render::prelude::*;
use bevy_ecs::prelude::*;
use bevy_transform::components::Transform;
use hexx::Hex;

use crate::components::Chunk;

#[derive(Bundle)]
pub struct ChunkBundle {
    pub chunk: Chunk,
}

impl ChunkBundle {
    pub fn new(hex: hexx::Hex) -> ChunkBundle {
        ChunkBundle {
            chunk: Chunk(hex)
        }
    }
}

#[derive(Bundle)]
pub struct ChunkDisplayBundle {
    pub chunk: Chunk,
    pub spatial: SpatialBundle
}

impl ChunkDisplayBundle {
    pub fn new(chunk: Hex, pos: Vec2) -> ChunkDisplayBundle {
        ChunkDisplayBundle {
            chunk: Chunk(chunk),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(pos.x, 0.0, pos.y),
                ..Default::default()
            },
        }
    }
}