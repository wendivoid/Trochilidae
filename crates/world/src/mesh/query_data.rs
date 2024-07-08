use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_utils::HashMap;
use hexx::Hex;

use crate::HexMap;

use crate::components::*;

#[derive(SystemParam)]
pub struct ChunkQueryData<'w, 's> {
    data_query: Query<'w, 's, (&'static Elevation, &'static CellColor)>,
}

impl<'w, 's> ChunkQueryData<'w, 's> {
    pub fn data<'a>(&self, map: &HexMap, cells: impl Iterator<Item = &'a Hex>) -> HashMap<Hex, (f32, Color)> {
        cells
            .map(|c| {
                map
                    .entities
                    .get(c)
                    .map(|x| self.data_query.get(*x).ok().map(|(y, z)| (*c, (y.0, z.0))))
                    .unwrap_or_default()
            })
            .flatten()
            .collect()
    }
}