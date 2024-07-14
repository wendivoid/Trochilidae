use hexx::Hex;
use bevy_ecs::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct WorldOrigin {
    pub chunk: Option<Hex>,
    pub hex: Option<Hex>
}
