use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_render::prelude::*;

use world::core::components::ViewPort;

use crate::BioSphereEntity;

pub fn spawn_biosphere(mut commands: Commands, query: Query<Entity, With<ViewPort>>) {
    commands.init_resource::<crate::cache::PhenotypeCache>();
    commands.insert_resource(crate::biosphere::BioSphere::with_defaults());
    commands.entity(query.single()).with_children(|commands| {
        commands.spawn((
            BioSphereEntity,
            bevy_core::Name::new("BioSphere"),
            SpatialBundle::default(),
        ));
    });
}
