use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_math::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
//use bevy_panorbit_camera::PanOrbitCamera;
use bevy_transform::components::Transform;

use crate::core::bundles::ViewportChunkBundle;
use crate::core::components::{Canvas, ViewportChunk};
use crate::terrain::Chunk;
use crate::ChunkDescriptor;
use crate::InsertWorldChunk;
use crate::{
    observer::{Observer, WorldOrigin},
    EntityCache, WorldSettings,
};

pub fn update_viewport(
    mut cached_chunk: Local<Option<hexx::Hex>>,
    mut commands: Commands,
    settings: Res<WorldSettings>,
    origin: Res<WorldOrigin>,
    mut cache: ResMut<EntityCache>,
    observer: Query<&PanOrbitCamera, With<Observer>>,
    mut events: EventWriter<InsertWorldChunk>,
    canvas: Query<Entity, With<Canvas>>,
    chunks: Query<(Entity, &Chunk), With<ViewportChunk>>,
) {
    if origin.is_changed() && !observer.is_empty() {
        *cached_chunk = origin.chunk;
        let observer_pos = observer.single().target_focus.xz();
        let mut visible_chunks = vec![];
        for (chunk, cells) in settings.visible_chunks(observer_pos) {
            visible_chunks.push(chunk);
            let descriptor = ChunkDescriptor::new(&settings, chunk, cells.len());
            if cache.contains_key(&chunk) {
                commands
                    .entity(cache[&chunk])
                    .insert(Transform::from_translation(descriptor.world_position()));
            } else {
                commands.entity(canvas.single()).with_children(|commands| {
                    let entity = commands.spawn(ViewportChunkBundle::new(&descriptor)).id();
                    events.send(InsertWorldChunk {
                        entity,
                        descriptor,
                        cells,
                    });
                });
            }
        }
        // Despawn any chunks that arent visible anymore.
        for (entity, chunk) in chunks.iter() {
            if !visible_chunks.contains(&chunk.0) {
                if let Some(entity) = commands.get_entity(entity) {
                    entity.despawn_recursive();
                }
                cache.inner.remove(&chunk.0);
            }
        }
    }
}
