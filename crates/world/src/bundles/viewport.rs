use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_render::prelude::SpatialBundle;

use crate::components::ViewPort;

#[derive(Bundle)]
pub struct ViewPortBundle {
    pub viewport: ViewPort,
    pub name: Name,
    pub spatial: SpatialBundle,
}

impl Default for ViewPortBundle {
    fn default() -> Self {
        Self {
            viewport: Default::default(),
            name: Name::new("View Port"),
            spatial: Default::default(),
        }
    }
}
