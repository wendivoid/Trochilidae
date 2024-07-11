use bevy_asset::Handle;
use bevy_pbr::{wireframe::WireframeColor, StandardMaterial};
use bevy_render::prelude::*;
use bevy_ecs::prelude::*;
use bevy_tasks::Task;
use bevy_transform::components::Transform;
use hexx::Hex;

use crate::{components::{Chunk, MoistureMeshHandle, TerrainMeshHandle, WaterMeshHandle}, mesh::MeshDescriptor, utils::random_color, CHUNK_MATERIAL};

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
    pub terrain_handle: TerrainMeshHandle,
    pub water_handle: WaterMeshHandle,
    pub moisture_handle: MoistureMeshHandle,
    pub material: Handle<StandardMaterial>,
    pub wireframe_color: WireframeColor,
}

impl ChunkDisplayBundle {
    pub fn new(descriptor: &MeshDescriptor, terrain: Task<Mesh>, water: Task<Mesh>, moisture: Task<Mesh>) -> ChunkDisplayBundle {
        ChunkDisplayBundle {
            material: CHUNK_MATERIAL,
            chunk: Chunk(descriptor.chunk),
            wireframe_color: WireframeColor { color: random_color() },
            terrain_handle: TerrainMeshHandle::new(terrain, descriptor.cachable),
            water_handle: WaterMeshHandle::new(water),
            moisture_handle: MoistureMeshHandle::new(moisture),
            spatial: SpatialBundle {
                transform: Transform::from_translation(descriptor.world_position()),
                ..Default::default()
            },
        }
    }
}