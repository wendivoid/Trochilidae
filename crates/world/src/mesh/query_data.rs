use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_utils::HashMap;
use hexx::Hex;

use crate::EntityCache;

use crate::components::*;
use crate::WorldSettings;

use super::TerrainMeshBuilder;

#[derive(SystemParam)]
pub struct ChunkQueryData<'w, 's> {
    data_query: Query<'w, 's, (&'static Elevation, &'static CellColor, &'static WaterTable, &'static Moisture)>,
}

impl<'w, 's> ChunkQueryData<'w, 's> {
    pub fn data<'a>(
        &self,
        map: &EntityCache,
        cells: impl Iterator<Item = &'a (Hex, Hex)>,
    ) -> (HashMap<Hex, (f32, Color, f32, f32)>, HashMap<Hex, f32>, HashMap<Hex, f32>) {
        let cells: HashMap<Hex, (f32, Color, f32, f32)> = cells
            .map(|(wrapped, _)| {
                map.inner
                    .get(wrapped)
                    .map(|x| self.data_query.get(*x).ok().map(|(y, z, q, t)| (*wrapped, (y.0, z.0, q.0, t.0))))
                    .unwrap_or_default()
            })
            .flatten()
            .collect();

        let mut water = HashMap::new();
        let mut moisture = HashMap::new();
        for (cell, (e, _, w, t)) in cells.iter() {
            if w > e {
               water.insert(*cell, *w);
            }
            if *t > 0.5 {
                moisture.insert(*cell, *t);
            }
        }
        (cells, water, moisture)
    }

    pub fn builder<'a>(
        &self,
        center: Hex,
        settings: &WorldSettings,
        entities: HashMap<Hex, (f32, Color, f32, f32)>,
    ) -> TerrainMeshBuilder {
        TerrainMeshBuilder::new(center, entities, &settings)
    }
}
