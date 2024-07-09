use bevy_ecs::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct WorldOrigin {
    pub active: Option<hexx::Hex>
}