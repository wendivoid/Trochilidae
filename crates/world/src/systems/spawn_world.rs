use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_utils::HashMap;

use crate::{bundles::{ChunkBundle, HexWorldBundle}, compose::CellComposer, HexMap, WorldSettings};

pub fn spawn_world(
    mut commands: Commands,
    mut cell_composer: ResMut<CellComposer>,
    settings: Res<WorldSettings>,
) {
    let mut entities = HashMap::new();
    commands.spawn(HexWorldBundle::default()).with_children(|commands| {
        for (chunk, children) in settings.all_chunks() {
            let c = commands.spawn(ChunkBundle::new(chunk)).id();
            for child in children {
                let bundle = cell_composer.0.compose_cell(child);
                let h = commands.spawn(bundle).set_parent(c).id();
                entities.insert(child, h);
            }
        }
    });
    commands.insert_resource(HexMap {
        entities,
        cache: Default::default()
    });
}