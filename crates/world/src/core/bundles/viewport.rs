use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_render::prelude::SpatialBundle;

use crate::core::components::ViewPort;

#[derive(Bundle)]
pub struct ViewPortBundle {
    pub viewport: ViewPort,
    pub name: Name,
    pub spatial: SpatialBundle,
}

impl Default for ViewPortBundle {
    fn default() -> Self {
        Self {
            name: Name::new("View Port"),
            viewport: Default::default(),
            spatial: Default::default(),
        }
    }
}
