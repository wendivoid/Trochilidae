use bevy::prelude::*;

mod utils;

use plants::{
    material::PlantMaterial,
    PlantBundle,
    mesh::MeshRenderer,
};
use sickle_ui::prelude::Dropdown;

#[derive(Component)]
pub struct Plant {
    pub lsystem: plants::PlantSystem,
    pub cfg: plants::mesh::MeshRenderConfig,
}


fn main() {
    App::new()
        .add_plugins(utils::ExamplePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (mesh_lsystem, lsystem_changed))
        .run()
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<PlantMaterial>>) {
    commands.spawn(PlantBundle {
            material: materials.add(PlantMaterial { time_scale: 1.0 }),
            ..Default::default()
        })
        .insert(Plant {
            lsystem: plants::monopodial(),
            cfg: Default::default(),
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

fn lsystem_changed(
    mut commands: Commands,
    plant_entity: Query<Entity, With<Plant>>,
    query: Query<&Dropdown, Changed<Dropdown>>,
) {
    for dropdown in query.iter() {
        if let Some(value) = dropdown.value() {
            let plant = match value {
                0 => Plant {
                    lsystem: plants::monopodial(),
                    cfg: Default::default(),
                },
                1 => Plant {
                    lsystem: plants::sympodial(),
                    cfg: Default::default(),
                },
                2 => Plant {
                    lsystem: plants::ternary(),
                    cfg: Default::default(),
                },
                _ => unreachable!(),
            };
            commands.entity(plant_entity.single()).insert(plant);
        }
    }
}
