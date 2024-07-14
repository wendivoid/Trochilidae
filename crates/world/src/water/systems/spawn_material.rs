use bevy_asset::prelude::*;
use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_pbr::prelude::*;
use bevy_render::alpha::AlphaMode;

use crate::water::WATER_MATERIAL;

pub fn spawn_material(mut materials: ResMut<Assets<StandardMaterial>>) {
    materials.insert(
        WATER_MATERIAL.id(),
        StandardMaterial {
            alpha_mode: AlphaMode::Blend,
            base_color: Color::srgba(0.86, 0.86, 0.86, 1.0),
            ..Default::default()
        },
    );
}
