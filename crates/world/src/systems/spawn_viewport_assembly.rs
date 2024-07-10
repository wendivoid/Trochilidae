use bevy_ecs::prelude::*;
use bevy_hierarchy::BuildChildren;

use crate::{bundles::{CanvasBundle, ObserverBundle, ViewPortBundle}, core::ObserverDiagnostics};

pub fn spawn_viewport_assembly(mut commands: Commands) {
    commands.insert_resource(ObserverDiagnostics { 
        hex: hexx::Hex::ZERO,
        chunk: hexx::Hex::ZERO
    });
    commands
        .spawn(ViewPortBundle::default())
        .with_children(|parent| {
            parent.spawn(ObserverBundle::default());
            parent.spawn(CanvasBundle::default());
        });
}
