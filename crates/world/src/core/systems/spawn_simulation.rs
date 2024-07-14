use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_utils::HashMap;
use blueprint::Blueprint;

use crate::{
    core::bundles::{ChunkBundle, HexWorldBundle},
    core::compose::{BlueprintComposer, ComposeCell},
    EntityCache, EntityMap, WorldSettings,
};

pub fn spawn_simulation(mut commands: Commands, settings: Res<WorldSettings>) {
    let mut cell_composer = BlueprintComposer(Blueprint::simple().unwrap());
    let mut entities = HashMap::new();
    commands
        .spawn(HexWorldBundle::default())
        .with_children(|commands| {
            for (chunk_hex, children) in settings.all_chunks() {
                commands
                    .spawn(ChunkBundle::new(chunk_hex))
                    .with_children(|parent| {
                        for child in children {
                            let bundle = cell_composer.compose_cell(child);
                            let h = parent.spawn(bundle).id();
                            entities.insert(child, h);
                        }
                    });
            }
        });
    commands.insert_resource(EntityMap::new(entities));
    commands.insert_resource(EntityCache::default());
}
