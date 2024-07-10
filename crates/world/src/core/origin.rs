use bevy_ecs::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct WorldOrigin {
    pub chunk: Option<hexx::Hex>,
    pub hex: Option<hexx::Hex>
}