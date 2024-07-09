use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_utils::HashMap;
use hexx::Hex;

use crate::EntityCache;

use crate::components::*;
use crate::WorldSettings;

use super::ChunkMeshBuilder;

#[derive(SystemParam)]
pub struct ChunkQueryData<'w, 's> {
    data_query: Query<'w, 's, (&'static Elevation, &'static CellColor)>,
}

impl<'w, 's> ChunkQueryData<'w, 's> {
    pub fn data<'a>(
        &self,
        map: &EntityCache,
        cells: impl Iterator<Item = &'a Hex>,
    ) -> HashMap<Hex, (f32, Color)> {
        cells
            .map(|c| {
                map.inner
                    .get(c)
                    .map(|x| self.data_query.get(*x).ok().map(|(y, z)| (*c, (y.0, z.0))))
                    .unwrap_or_default()
            })
            .flatten()
            .collect()
    }

    pub fn builder<'a>(
        &self,
        center: Hex,
        settings: &WorldSettings,
        entities: HashMap<Hex, (f32, Color)>,
    ) -> ChunkMeshBuilder {
        ChunkMeshBuilder::new(center, entities, &settings)
    }
}
