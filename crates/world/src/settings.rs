use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use hexx::{Hex, HexBounds, HexLayout};
use itertools::Itertools;

#[derive(Resource)]
pub struct WorldSettings {
    pub hex_radius: f32,
    pub chunk_radius: u32,
    pub world_radius: u32,
    pub visible_radius: u32,
}

impl WorldSettings {
    pub fn layout(&self) -> HexLayout {
        HexLayout {
            hex_size: Vec2::splat(self.hex_radius),
            ..Default::default()
        }
    }

    pub fn bounds(&self) -> HexBounds {
        HexBounds::new(Hex::ZERO, self.world_radius)
    }

    pub fn chunk_bounds(&self) -> HexBounds {
        HexBounds::new(Hex::ZERO, self.chunk_radius)
    }

    pub fn all_coords(&self) -> impl Iterator<Item = Hex> {
        self.bounds().all_coords()
    }

    pub fn all_chunks(&self) -> impl Iterator<Item = (Hex, Vec<Hex>)> {
        self.all_coords()
            .map(|hex| (hex.to_lower_res(self.chunk_radius), hex))
            .into_group_map()
            .into_iter()
    }

    pub fn visible_chunks(&self, pos: Vec2) -> impl Iterator<Item = (Hex, Vec<(Hex, Hex)>)> {
        let o_hex = self.layout().world_pos_to_hex(pos);
        let bounds = self.bounds();
        o_hex
            .range(self.visible_radius)
            .map(|hex| (hex.to_lower_res(self.chunk_radius), (bounds.wrap(hex), hex)))
            .into_group_map()
            .into_iter()
    }

    pub fn chunk_hex_count(&self) -> usize {
        self.chunk_bounds().hex_count()
    }
}

impl Default for WorldSettings {
    fn default() -> WorldSettings {
        WorldSettings {
            hex_radius: 3.0,
            chunk_radius: 5,
            world_radius: 150,
            visible_radius: 45,
        }
    }
}
