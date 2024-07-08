use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_render::prelude::*;

use crate::components::Canvas;

#[derive(Bundle)]
pub struct CanvasBundle {
    pub name: Name,
    pub canvas: Canvas,
    pub spatial: SpatialBundle,
}

impl Default for CanvasBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Canvas"),
            canvas: Canvas,
            spatial: SpatialBundle::default(),
        }
    }
}
