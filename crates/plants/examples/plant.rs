use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use plants::{mesh::{MeshRenderConfig, MeshRenderer}, PlantSystem};
use sickle_ui::{prelude::*, SickleUiPlugin};

#[derive(Component)]
struct Plant {
    pub lsystem: PlantSystem,
    pub cfg: MeshRenderConfig
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PanOrbitCameraPlugin,
            InfiniteGridPlugin,
            WireframePlugin,
            SickleUiPlugin,
        ))
        .insert_resource(WireframeConfig {
            global: false,
            default_color: Color::GOLD,
        })
        .add_systems(Startup, (build_ui, setup))
        .add_systems(Update, (mesh_lsystem, lsystem_changed, toggle_wireframe))
        .run()
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(InfiniteGridBundle::default());
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(10.0, 10.0, 10.0),
            ..default()
        })
        .insert(PanOrbitCamera::default());
    let mono = plants::monopodial();

    commands.spawn(PbrBundle {
        material: materials.add(StandardMaterial {
            unlit: true,
            ..Default::default()
        }),
        ..Default::default()
    }).insert(Plant { lsystem: mono, cfg: Default::default() });
}

fn build_ui(mut commands: Commands) {
    let systems = vec!["Monopodial Tree", "Sympodial Tree", "Ternary Tree",  "Cordate Leaf", "Simple Leaf", "Rose Leaf"];
    commands.ui_builder(UiRoot).column(|column| {
        column
            .style()
            .height(Val::Auto)
            .margin(UiRect::all(Val::Px(15.0)))
            .padding(UiRect::all(Val::Px(5.0)))
            .background_color(Color::rgba(0.0, 0.0, 0.0, 5.0));
        column.row(|row| {
            row.checkbox(Some("Wireframe".into()), true);
        });
        column.row(|row| {
            row.dropdown(systems, Some(0));
        });
    });
}

fn mesh_lsystem(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, query: Query<(Entity, &Plant), Changed<Plant>>) {
    for (entity, plant) in query.iter() {
        let mesh = MeshRenderer::new(&plant.lsystem, plant.cfg.clone()).build();
        commands.entity(entity).insert(meshes.add(mesh));
    }
}

fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    query: Query<&Checkbox, Changed<Checkbox>>
) {
    for check in query.iter() {
        wireframe_config.global = check.checked;
    }
}

fn lsystem_changed(mut commands: Commands, plant_entity: Query<Entity, With<Plant>>, query: Query<&Dropdown, Changed<Dropdown>>) {
    for dropdown in query.iter() {
        if let Some(value) = dropdown.value() {
            let plant = match value {
                0 => {
                    Plant { lsystem: plants::monopodial(), cfg: Default::default() }
                },
                1 => {
                    Plant { lsystem: plants::sympodial(), cfg: Default::default() }
                },
                2 => {
                    Plant { lsystem: plants::ternary(), cfg: Default::default() }
                }
                3 => {
                    Plant { lsystem: plants::cordate(), cfg: MeshRenderConfig { angle: 16.0, age: 12, length: 0.1, ..Default::default() }  }
                }
                4 => {
                    Plant { lsystem: plants::simple_leaf(), cfg: MeshRenderConfig { angle: 60.0, age: 20, length: 0.1, ..Default::default() } }
                }
                5 => {
                    Plant { lsystem: plants::rose_leaf(), cfg: MeshRenderConfig { angle: 60.0, age: 25, length: 0.1, ..Default::default() } }
                }
                _ => unreachable!()
            };
            commands.entity(plant_entity.single()).insert(plant);
        }
    }
}