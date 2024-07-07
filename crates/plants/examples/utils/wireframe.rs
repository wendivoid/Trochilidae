use bevy::prelude::*;
use bevy_pbr::wireframe::WireframeConfig;
use sickle_ui::prelude::Checkbox;

pub struct WireframePlugin;

impl Plugin for WireframePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::pbr::wireframe::WireframePlugin);
        app.insert_resource(WireframeConfig {
            global: true,
            default_color: bevy_color::palettes::tailwind::AMBER_400.into(),
        });
        app.add_systems(Update, toggle_wireframe);
    }
}

fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    query: Query<&Checkbox, Changed<Checkbox>>,
) {
    for check in query.iter() {
        wireframe_config.global = check.checked;
    }
}