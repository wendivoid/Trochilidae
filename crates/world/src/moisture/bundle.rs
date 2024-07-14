use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_render::prelude::*;

#[derive(Bundle, Debug)]
pub struct MoistureBundle {
    pub name: Name,
    pub spatial: SpatialBundle,
}

impl Default for MoistureBundle {
    fn default() -> MoistureBundle {
        MoistureBundle {
            name: Name::new("Moisture"),
            spatial: Default::default(),
        }
    }
}
