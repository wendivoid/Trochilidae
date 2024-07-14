use bevy_ecs::prelude::*;
use hexx::Hex;

#[derive(Debug, Default, Resource)]
pub struct WorldOrigin {
    pub chunk: Option<Hex>,
    pub hex: Option<Hex>,
}
