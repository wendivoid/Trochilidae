use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_gizmos::{
    light::{LightGizmoColor, LightGizmoConfigGroup},
    prelude::GizmoConfigStore,
};
use bevy_core::prelude::*;
use bevy_math::Vec3;
use bevy_pbr::prelude::*;
use bevy_transform::prelude::*;
use bevy_hierarchy::prelude::*;

use crate::{components::ViewPort, sky::Sun, WorldSettings};

pub fn spawn(
    mut commands: Commands,
    settings: Res<WorldSettings>,
    mut config_store: ResMut<GizmoConfigStore>,
    viewport: Query<Entity, With<ViewPort>>,
) {
    let (_, light_config) = config_store.config_mut::<LightGizmoConfigGroup>();
    light_config.draw_all = true;
    light_config.color = LightGizmoColor::MatchLightColor;
    let past_edge = settings.world_radius as f32 * (settings.hex_radius * 2.0);
    commands.entity(viewport.single()).with_children(|commands| {
        commands.spawn((
            DirectionalLightBundle {
                directional_light: DirectionalLight {
                    color: Color::srgba(0.9, 0.9, 0.95, 1.0),
                    shadows_enabled: true,
                    ..Default::default()
                },
                transform: Transform::from_xyz(past_edge, 50.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            },
            Sun,
            Name::new("Sun")
        ));
    });
}
