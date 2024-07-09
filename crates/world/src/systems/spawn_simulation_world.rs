use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_utils::HashMap;

use crate::{
    bundles::{ChunkBundle, HexWorldBundle},
    compose::CellComposer,
    core::{MeshCache, WorldSettings},
    EntityCache,
};

pub fn spawn_simulation_world(
    mut commands: Commands,
    mut cell_composer: ResMut<CellComposer>,
    settings: Res<WorldSettings>,
) {
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
    commands.insert_resource(EntityCache::new(entities));
    commands.insert_resource(MeshCache::default());
}
