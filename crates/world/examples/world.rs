use bevy::input::common_conditions::input_toggle_active;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "../../assets".into(),
                ..Default::default()
            }),
            WireframePlugin,
            bevy::dev_tools::fps_overlay::FpsOverlayPlugin::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F12)),
            world::WorldPlugin::default(),
        ))
        .insert_resource(Msaa::Off)
        .insert_resource(WireframeConfig {
            global: false,
            default_color: bevy_color::palettes::css::WHITE.into(),
        })
        .insert_resource(Msaa::Off)
        .run();
}
