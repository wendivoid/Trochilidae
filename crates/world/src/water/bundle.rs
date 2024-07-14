use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_render::prelude::*;

#[derive(Bundle)]
pub struct WaterBundle {
    pub name: Name,
    pub spatial: SpatialBundle,
}

impl Default for WaterBundle {
    fn default() -> WaterBundle {
        WaterBundle {
            name: Name::new("Water"),
            spatial: Default::default(),
        }
    }
}
