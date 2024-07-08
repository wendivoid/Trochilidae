use bevy_utils::HashMap;
use bevy_ecs::prelude::*;

#[derive(Debug, Resource)]
pub struct HexMap {
    pub cache: HashMap<hexx::Hex, Entity>,
    pub entities: HashMap<hexx::Hex, Entity>,
}

#[derive(Debug, Default, Resource)]
pub struct HexState {
    pub active: Option<hexx::Hex>
}