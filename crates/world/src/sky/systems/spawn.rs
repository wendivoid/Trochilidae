use bevy_ecs::prelude::*;
use bevy_gizmos::{
    light::{LightGizmoColor, LightGizmoConfigGroup},
    prelude::GizmoConfigStore,
};
use bevy_pbr::prelude::*;
use bevy_transform::prelude::*;
use bevy_hierarchy::prelude::*;

use crate::{components::ViewPort, sky::Sun};

pub fn spawn(
    mut commands: Commands,
    mut config_store: ResMut<GizmoConfigStore>,
    viewport: Query<Entity, With<ViewPort>>,
) {
    let (_, light_config) = config_store.config_mut::<LightGizmoConfigGroup>();
    light_config.draw_all = true;
    light_config.color = LightGizmoColor::MatchLightColor;
    commands.entity(viewport.single()).with_children(|commands| {
        commands.spawn((
            DirectionalLightBundle {
                transform: Transform::from_xyz(0.0, 30.0, 0.0),
                ..Default::default()
            },
            Sun, // Marks the light as Sun
        ));
    });
}
