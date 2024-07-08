use bevy::prelude::*;
use bevy_pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WireframePlugin,
            bevy::dev_tools::fps_overlay::FpsOverlayPlugin::default(),
            WorldInspectorPlugin::new(),
            world::WorldPlugin::default()
        ))
        .insert_resource(WireframeConfig {
            global: true,
            default_color: bevy_color::palettes::css::WHITE.into()
        })
        .run();
}