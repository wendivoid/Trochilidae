use bevy_asset::prelude::*;
use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_pbr::prelude::*;

use crate::terrain::TERRAIN_MATERIAL;

pub fn spawn_material(
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    materials.insert(
        TERRAIN_MATERIAL.id(),
        StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 7.0 / 10.0,
            metallic: 7.0 / 4.0,
            ..Default::default()
        },
    );
}