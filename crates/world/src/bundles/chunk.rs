use bevy_asset::Handle;
use bevy_pbr::{wireframe::WireframeColor, StandardMaterial};
use bevy_render::prelude::*;
use bevy_ecs::prelude::*;
use bevy_tasks::Task;
use bevy_transform::components::Transform;
use hexx::Hex;

use crate::{components::{Chunk, MeshHandle}, mesh::MeshDescriptor, utils::random_color, CHUNK_MATERIAL};

#[derive(Bundle)]
pub struct ChunkBundle {
    pub chunk: Chunk,
}

impl ChunkBundle {
    pub fn new(hex: Hex) -> ChunkBundle {
        ChunkBundle {
            chunk: Chunk(hex)
        }
    }
}

#[derive(Bundle)]
pub struct ChunkDisplayBundle {
    pub chunk: Chunk,
    pub spatial: SpatialBundle,
    pub mesh_handle: MeshHandle,
    pub material: Handle<StandardMaterial>,
    pub wireframe_color: WireframeColor,
}

impl ChunkDisplayBundle {
    pub fn new(descriptor: &MeshDescriptor, task: Task<Mesh>) -> ChunkDisplayBundle {
        ChunkDisplayBundle {
            material: CHUNK_MATERIAL,
            chunk: Chunk(descriptor.chunk),
            wireframe_color: WireframeColor { color: random_color() },
            mesh_handle: MeshHandle::new(task, descriptor.cachable),
            spatial: SpatialBundle {
                transform: Transform::from_translation(descriptor.world_position()),
                ..Default::default()
            },
        }
    }
}