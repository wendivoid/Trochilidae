use bevy::{input::common_conditions::input_toggle_active, prelude::*};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_render::{
    batching::NoAutomaticBatching,
    view::{NoCpuCulling, NoFrustumCulling},
};
use bevy_utils::HashMap;
use plants::vascular::{
    mesh::MeshRenderer,
    render::{VascularData, VascularInstanceMap},
};

#[derive(Component)]
pub struct Plant {
    pub lsystem: plants::vascular::VascularLSystem,
    pub cfg: plants::vascular::mesh::MeshRenderConfig,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "../../assets".into(),
                ..Default::default()
            }),
            bevy_pbr::wireframe::WireframePlugin,
            PanOrbitCameraPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F12)),
            bevy::dev_tools::fps_overlay::FpsOverlayPlugin::default(),
        ))
        .add_plugins(plants::vascular::render::VascularMaterialPlugin::default())
        .add_systems(Startup, (setup, setup_world))
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let creatures = vec![
        plants::vascular::monopodial(),
        plants::vascular::sympodial()
    ];
    for (dex, creature) in creatures.iter().enumerate() {
        let mut instances = HashMap::new();
        for x in -5..5 {
            instances.insert(
                hexx::Hex::new(x as i32, x as i32),
                VascularData {
                    birthdate: x as f32,
                    scale: 1.0,
                    position: Vec3::new(x as f32 * 4.0, 0.0, dex as f32 * 8.0),
                },
            );
        }
        let cfg = Default::default();
        let mesh = MeshRenderer::new(&creature, cfg).build();
        commands
            .spawn(SpatialBundle::default())
            .insert(VascularInstanceMap(instances))
            .insert(NoAutomaticBatching)
            .insert(NoFrustumCulling)
            .insert(NoCpuCulling)
            .insert(bevy::pbr::wireframe::Wireframe)
            .insert(meshes.add(mesh));
    }
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(15.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(10.0, 10.0, 10.0),
            ..default()
        })
        .insert(bevy_core_pipeline::prepass::DepthPrepass)
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
