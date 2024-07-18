use bevy_asset::prelude::*;
use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_pbr::prelude::*;

use crate::moisture::MOISTURE_MATERIAL;

pub fn spawn_material(mut materials: ResMut<Assets<StandardMaterial>>) {
    materials.insert(
        MOISTURE_MATERIAL.id(),
        StandardMaterial {
            base_color: Color::srgba(0.9, 0.9, 0.95, 0.95),
            ..Default::default()
        },
    );
}
