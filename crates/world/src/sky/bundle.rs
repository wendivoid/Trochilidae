use bevy_color::prelude::*;
use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_pbr::prelude::*;
use bevy_transform::prelude::*;

use crate::WorldSettings;

use super::Sun;

#[derive(Bundle, Debug)]
pub struct SunBundle {
    pub sun: Sun,
    pub name: Name,
    pub light: DirectionalLightBundle,
}

impl SunBundle {
    pub fn new(settings: &WorldSettings) -> SunBundle {
        let past_edge = settings.world_radius as f32 * (settings.hex_radius * 2.0);
        SunBundle {
            sun: Sun,
            name: Name::new("Sun"),
            light: DirectionalLightBundle {
                directional_light: DirectionalLight {
                    color: Color::srgba(0.9, 0.9, 0.95, 1.0),
                    shadows_enabled: true,
                    ..Default::default()
                },
                transform: Transform::from_xyz(past_edge, 50.0, 0.0)
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            },
        }
    }
}
