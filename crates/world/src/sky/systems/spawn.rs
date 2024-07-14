use bevy_ecs::prelude::*;
use bevy_gizmos::{
    light::{LightGizmoColor, LightGizmoConfigGroup},
    prelude::GizmoConfigStore,
};
use bevy_hierarchy::prelude::*;

use crate::{core::components::ViewPort, sky::SunBundle, WorldSettings};

pub fn spawn(
    mut commands: Commands,
    settings: Res<WorldSettings>,
    mut config_store: ResMut<GizmoConfigStore>,
    viewport: Query<Entity, With<ViewPort>>,
) {
    let (_, light_config) = config_store.config_mut::<LightGizmoConfigGroup>();
    light_config.draw_all = true;
    light_config.color = LightGizmoColor::MatchLightColor;
    commands.entity(viewport.single()).with_children(|commands| {
        commands.spawn(SunBundle::new(&settings));
    });
}
