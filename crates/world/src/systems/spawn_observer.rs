use bevy_ecs::prelude::*;
use bevy_hierarchy::BuildChildren;

use crate::bundles::{ViewPortBundle, ObserverBundle, CanvasBundle};

pub fn spawn_observer(mut commands: Commands) {
    let viewport = commands.spawn(ViewPortBundle::default()).id();
    commands.spawn(ObserverBundle::default()).set_parent(viewport);
    commands.spawn(CanvasBundle::default()).set_parent(viewport);
}