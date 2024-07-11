use bevy_asset::prelude::*;
use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_pbr::prelude::*;
use bevy_render::alpha::AlphaMode;
use bevy_water::material::{StandardWaterMaterial, WaterMaterial};

use crate::{CHUNK_MATERIAL, MOISTURE_MATERIAL, WATER_MATERIAL};

pub fn spawn_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut water: ResMut<Assets<StandardWaterMaterial>>,
) {
    materials.insert(
        CHUNK_MATERIAL.id(),
        StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 7.0 / 10.0,
            metallic: 7.0 / 4.0,
            ..Default::default()
        },
    );
    materials.insert(
        MOISTURE_MATERIAL.id(),
        StandardMaterial {
            base_color: Color::WHITE,
            ..Default::default()
        },
    );
    water.insert(
        WATER_MATERIAL.id(),
        StandardWaterMaterial {
            base: StandardMaterial {
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            },
            extension: WaterMaterial {
                edge_scale: 0.1,
                ..Default::default()
            },
        },
    );
}
