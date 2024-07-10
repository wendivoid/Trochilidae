use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WireframePlugin,
            bevy::dev_tools::fps_overlay::FpsOverlayPlugin::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F12)),
            world::WorldPlugin::default()
        ))
        .insert_resource(WireframeConfig {
            global: false,
            default_color: bevy_color::palettes::css::WHITE.into()
        })
        .run();
}