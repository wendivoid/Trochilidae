use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_render::prelude::SpatialBundle;

use crate::components::Stage;

#[derive(Bundle)]
pub struct ViewPortBundle {
    pub stage: Stage,
    pub name: Name,
    pub spatial: SpatialBundle,
}

impl Default for ViewPortBundle {
    fn default() -> Self {
        Self {
            stage: Default::default(),
            name: Name::new("View Port"),
            spatial: Default::default(),
        }
    }
}
