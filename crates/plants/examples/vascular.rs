use bevy::{input::common_conditions::input_toggle_active, prelude::*};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy::pbr::wireframe::WireframePlugin;
use plants::vascular::mesh::MeshRenderer;

#[derive(Component)]
pub struct Plant {
    pub lsystem: plants::vascular::VascularLSystem,
    pub cfg: plants::vascular::mesh::MeshRenderConfig,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PanOrbitCameraPlugin,
            WireframePlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F12)),
            bevy::dev_tools::fps_overlay::FpsOverlayPlugin::default(),
        ))
        .add_systems(Startup, (setup, setup_world))
        .add_systems(Update, mesh_lsystem)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(SpatialBundle::default())
        .insert(Plant {
            lsystem: plants::vascular::sympodial(),
            cfg: Default::default(),
        });
}

fn setup_world(
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

fn mesh_lsystem(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &Plant), Changed<Plant>>,
) {
    for (entity, plant) in query.iter() {
        let mesh = MeshRenderer::new(&plant.lsystem, plant.cfg.clone()).build();
        commands.entity(entity).insert(meshes.add(mesh));
    }
}
