mod ui;
mod wireframe;

use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_asset::AssetPlugin;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "../../assets".into(),
                ..Default::default()
            }),
            PanOrbitCameraPlugin,
            plants::PlantPlugin,
            ui::ExampleUiPlugin,
            wireframe::WireframePlugin,
        ));
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(8.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(10.0, 10.0, 10.0),
            ..default()
        })
        .insert(PanOrbitCamera::default());

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
