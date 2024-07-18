use bevy_asset::Handle;
use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_pbr::StandardMaterial;
use bevy_render::prelude::*;
use bevy_transform::components::Transform;
use hexx::Hex;

use crate::{
    core::components::ViewportChunk,
    terrain::{Chunk, TERRAIN_MATERIAL},
    ChunkDescriptor,
};

#[derive(Bundle)]
pub struct ChunkBundle {
    pub name: Name,
    pub chunk: Chunk,
}

impl ChunkBundle {
    pub fn new(hex: Hex) -> ChunkBundle {
        ChunkBundle {
            chunk: Chunk(hex),
            name: Name::new(format!("Hex Chunk({}, {})", hex.x, hex.y)),
        }
    }
}

#[derive(Bundle)]
pub struct ViewportChunkBundle {
    pub chunk: Chunk,
    pub viewport_chunk: ViewportChunk,
    pub spatial: SpatialBundle,
    pub material: Handle<StandardMaterial>,
}

impl ViewportChunkBundle {
    pub fn new(descriptor: &ChunkDescriptor) -> ViewportChunkBundle {
        ViewportChunkBundle {
            viewport_chunk: ViewportChunk,
            material: TERRAIN_MATERIAL,
            chunk: Chunk(descriptor.chunk),
            spatial: SpatialBundle {
                transform: Transform::from_translation(descriptor.world_position()),
                ..Default::default()
            },
        }
    }
}
