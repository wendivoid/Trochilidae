use bevy_core::prelude::*;
use bevy_ecs::prelude::*;

use crate::core::components::HexWorld;

#[derive(Bundle)]
pub struct HexWorldBundle {
    pub name: Name,
    pub hex_world: HexWorld,
}

impl Default for HexWorldBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Hex World"),
            hex_world: HexWorld,
        }
    }
}
