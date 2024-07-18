use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_math::prelude::*;
use bevy_render::{
    batching::NoAutomaticBatching,
    prelude::*,
    view::{NoCpuCulling, NoFrustumCulling},
};
use bevy_utils::HashMap;
use hexx::Hex;
use plants::vascular::render::{VascularData, VascularInstanceData};
use world::{terrain::Elevation, InsertWorldChunk, WorldSettings};

use crate::{
    biosphere::BioSphere,
    cache::PhenotypeCache,
    phenotype::{Anatomy, PhenotypeId},
    BioSphereEntity, PhenotypeInstance,
};

pub fn update_visible_instances(
    mut commands: Commands,
    map: Res<world::EntityMap>,
    biosphere: Res<BioSphere>,
    mut meshes: ResMut<Assets<Mesh>>,
    world_settings: Res<WorldSettings>,
    biosphere_entity: Query<Entity, With<BioSphereEntity>>,
    mut cache: ResMut<PhenotypeCache>,
    mut events: EventReader<InsertWorldChunk>,
    mut dis_data: Query<&mut VascularInstanceData>,
    sim_data: Query<(&PhenotypeInstance, &Elevation, &Parent)>,
) {
    for InsertWorldChunk { cells, .. } in events.read() {
        let mut species_to_spawn: HashMap<PhenotypeId, (Mesh, HashMap<Hex, VascularData>)> =
            HashMap::new();
        for (wrapped, hex) in cells.iter() {
            if let Ok((instance, elevation, _parent)) = sim_data.get(map.inner[wrapped]) {
                if cache.contains_key(&instance.id) {
                    if let Ok(mut instance_data) = dis_data.get_mut(cache[&instance.id]) {
                        let hex_pos = world_settings.layout().hex_to_world_pos(*hex);
                        instance_data.insert(
                            *hex,
                            VascularData {
                                position: Vec3::new(hex_pos.x, elevation.0 + 10.0, hex_pos.y),
                                scale: instance.scale,
                                birthdate: instance.birthdate,
                            },
                        );
                    }
                } else {
                    let phenotype = biosphere.registry.inner.get(&instance.id).unwrap();
                    let hex_pos = world_settings.layout().hex_to_world_pos(*hex);
                    match &phenotype.anatomy {
                        Anatomy::Vascular(lsys) => {
                            let mut new_data = HashMap::new();
                            new_data.insert(
                                *hex,
                                VascularData {
                                    position: Vec3::new(hex_pos.x, elevation.0 + 10.0, hex_pos.y),
                                    scale: instance.scale,
                                    birthdate: instance.birthdate,
                                },
                            );
                            if species_to_spawn.contains_key(&instance.id) {
                                species_to_spawn
                                    .get_mut(&instance.id)
                                    .unwrap()
                                    .1
                                    .extend(new_data);
                            } else {
                                let mesh = plants::vascular::mesh::MeshRenderer::new(
                                    &lsys,
                                    Default::default(),
                                )
                                .build();
                                species_to_spawn.insert(instance.id, (mesh, new_data));
                            }
                        }
                    }
                }
            }
        }
        for (id, (mesh, data)) in species_to_spawn.into_iter() {
            commands
                .entity(biosphere_entity.single())
                .with_children(|commands| {
                    let entity = commands
                        .spawn((
                            VascularInstanceData(data),
                            meshes.add(mesh),
                            NoCpuCulling,
                            NoAutomaticBatching,
                            NoFrustumCulling,
                            SpatialBundle::default(),
                        ))
                        .id();
                    cache.inner.insert(id, entity);
                });
        }
    }
}
