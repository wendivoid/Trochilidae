use bevy_ecs::prelude::*;
use bevy_pbr::prelude::*;
use bevy_asset::prelude::*;
use bevy_color::prelude::*;

use crate::CHUNK_MATERIAL;

pub fn spawn_chunk_material(mut materials: ResMut<Assets<StandardMaterial>>) {
    materials.insert(CHUNK_MATERIAL.id(), StandardMaterial {
        base_color: Color::WHITE,
        ..Default::default()
    });
}