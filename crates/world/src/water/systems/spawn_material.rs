use bevy_asset::prelude::*;
use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_pbr::StandardMaterial;

use crate::water::{StandardWaterMaterial, WaterMaterial, WATER_MATERIAL};

pub fn spawn_material(mut materials: ResMut<Assets<StandardWaterMaterial>>) {
    materials.insert(
        WATER_MATERIAL.id(),
        StandardWaterMaterial {
            base: StandardMaterial {
                alpha_mode: bevy_render::alpha::AlphaMode::Blend,
                ..Default::default()
            },
            extension: WaterMaterial {
                deep_color: LinearRgba::new(0.086, 0.407, 1.0, 1.0),
                shallow_color: LinearRgba::new(0.325, 0.807, 0.971, 0.725),
                edge_color: LinearRgba::WHITE,
                clarity: 0.25,
                edge_scale: 0.1,
            },
        },
    );
}
