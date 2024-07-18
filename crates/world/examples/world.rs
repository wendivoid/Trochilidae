use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(States, Debug, Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct ExampleState;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "../../assets".into(),
                ..default()
            }),
            bevy::dev_tools::fps_overlay::FpsOverlayPlugin::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F12)),
            world::WorldPlugin::new(Startup, ExampleState),
        ))
        .init_state::<ExampleState>()
        .insert_resource(Msaa::Sample4)
        .run();
}
