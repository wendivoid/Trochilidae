use bevy_ecs::prelude::*;

#[derive(Component, Debug, Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct Observer;

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub struct WASDController {
    pub scale: f32,
}

impl Default for WASDController {
    fn default() -> Self {
        WASDController { scale: 25.0 }
    }
}