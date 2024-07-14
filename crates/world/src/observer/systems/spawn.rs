use bevy_ecs::prelude::*;
use bevy_hierarchy::BuildChildren;

use crate::{core::bundles::{CanvasBundle, ViewPortBundle}, observer::ObserverBundle};

pub fn spawn_viewport_assembly(mut commands: Commands) {
    commands
        .spawn(ViewPortBundle::default())
        .with_children(|parent| {
            parent.spawn(ObserverBundle::default());
            parent.spawn(CanvasBundle::default());
        });
}