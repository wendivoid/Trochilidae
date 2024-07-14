use bevy_ecs::prelude::*;

#[derive(Component, Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct HexWorld;

#[derive(Component, Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct ViewPort;

#[derive(Component, Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct ViewportChunk;

#[derive(Component, Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct Canvas;
