use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_render::mesh::Mesh;
use bevy_hierarchy::prelude::*;
use bevy_tasks::Task;
use rand::Rng;

use crate::{bundles::ChunkDisplayBundle, mesh::MeshDescriptor};

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::srgb(
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
    )
}

pub trait CommandExt {
    fn spawn_chunk_display(&mut self, descriptor: &MeshDescriptor, terrain: Task<Mesh>, water: Task<Mesh>, moisture: Task<Mesh>, parent: Entity) -> Entity;
}

impl<'w, 's> CommandExt for Commands<'w, 's> {
    fn spawn_chunk_display(&mut self, descriptor: &MeshDescriptor, terrain: Task<Mesh>, water: Task<Mesh>, moisture: Task<Mesh>, parent: Entity) -> Entity {
        self.spawn(ChunkDisplayBundle::new(&descriptor, terrain, water, moisture)).set_parent(parent).id()
    }
}
