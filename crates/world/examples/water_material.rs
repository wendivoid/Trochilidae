//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{
    core_pipeline::prepass::DepthPrepass, input::common_conditions::input_toggle_active,
    pbr::NotShadowCaster, prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use world::water::{StandardWaterMaterial, WaterMaterial};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "../../assets".into(),
            ..Default::default()
        }))
        .add_plugins(bevy_panorbit_camera::PanOrbitCameraPlugin)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F12)),
        )
        .add_plugins(MaterialPlugin::<StandardWaterMaterial>::default())
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut toons: ResMut<Assets<StandardWaterMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Water Plane
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Circle::new(8.0)),
            material: toons.add(StandardWaterMaterial {
                base: StandardMaterial {
                    alpha_mode: AlphaMode::Add,
                    ..Default::default()
                },
                extension: WaterMaterial {
                    deep_color: LinearRgba::new(0.086, 0.407, 1.0, 1.0),
                    shallow_color: LinearRgba::new(0.325, 0.807, 0.971, 0.725),
                    edge_color: LinearRgba::WHITE,
                    clarity: 0.25,
                    edge_scale: 0.1,
                },
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.9, 0.0))
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            ..default()
        })
        .insert(NotShadowCaster);
    // Water Container
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cylinder::new(8.0, 2.0).mesh().without_caps()),
            material: materials.add(StandardMaterial {
                base_color: Color::srgba(0.5, 0.8, 0.5, 1.0),
                double_sided: true,
                cull_mode: None,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(NotShadowCaster);
    //
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(8.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgba(0.5, 0.8, 0.5, 1.0),
            double_sided: true,
            cull_mode: None,
            ..Default::default()
        }),
        transform: Transform::from_translation(Vec3::new(0.0, -1.0, 0.0))
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-10.5, 13.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(bevy_panorbit_camera::PanOrbitCamera::default())
        .insert(DepthPrepass);
}
