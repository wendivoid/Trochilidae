use bevy_ecs::prelude::*;
use bevy_utils::HashMap;
use hexx::{EdgeDirection, Hex};
use rand::{rngs::ThreadRng, seq::IteratorRandom, thread_rng, Rng};
use world::{moisture::*, terrain::*, water::*, WorldSettings};

use crate::{
    biosphere::BioSphere,
    environment::Environment,
    PhenotypeInstance,
};

pub fn genesis(
    mut commands: Commands,
    settings: Res<WorldSettings>,
    biosphere: ResMut<BioSphere>,
    mut query: Query<(
        Entity,
        &Elevation,
        &WaterTable,
        &Moisture,
        &Cell,
        Option<&PhenotypeInstance>,
    )>,
) {
    let cell_data = query
        .iter_mut()
        .map(|(e, t, w, m, c, p)| (c.0, (e, t.0, w.0, m.0, p)))
        .collect::<HashMap<Hex, (Entity, f32, f32, f32, Option<&PhenotypeInstance>)>>();
    for phenotype_id in biosphere.graph.nodes() {
        let phenotype = biosphere.registry.inner.get(&phenotype_id).unwrap();
        let cell_locations = spawn_genesis(9, 100, &*settings);
        for cell in cell_locations.iter() {
            let (e, t, w, m, p) = cell_data.get(cell).unwrap();
            if p.is_none() {
                let env = Environment { elevation: *t, water_table: *w, moisture: *m};
                if phenotype.habitability.should_spawn(env) {
                    let birthdate = thread_rng().gen_range(0.0..phenotype.lifespan);
                    let scale = thread_rng().gen_range(0.7..1.4);
                    commands.entity(*e).insert(PhenotypeInstance {
                        id: phenotype_id,
                        scale,
                        birthdate,
                    });
                }
            }
        }
    }
}

fn spawn_genesis(genesis_count: usize, cell_count: usize, settings: &WorldSettings) -> Vec<Hex> {
    use rand::thread_rng;
    let mut rng = thread_rng();
    let mut hexs = vec![];
    for _ in 1..=genesis_count {
        hexs.extend(drunkards_walk(&mut rng, cell_count, settings).into_iter());
    }
    hexs
}

fn drunkards_walk(rng: &mut ThreadRng, count: usize, settings: &WorldSettings) -> Vec<Hex> {
    let mut cells = vec![];

    let bounds = settings.bounds();

    let mut hex = settings.bounds().all_coords().choose(rng).unwrap();

    loop {
        if cells.len() != count {
            let dir =
                EdgeDirection::from_pointy_angle(rng.gen_range(0.0..std::f32::consts::PI * 2.0));
            let _hex = bounds.wrap(hex.neighbor(dir));
            if cells.contains(&_hex) {
                hex = _hex;
                continue;
            } else {
                hex = _hex;
                cells.push(bounds.wrap(hex));
            }
        } else {
            break;
        }
    }
    cells
}
