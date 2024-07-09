use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_render::mesh::Mesh;
use bevy_hierarchy::prelude::*;
use bevy_tasks::Task;
use bevy_transform::components::Transform;
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
    fn spawn_chunk_display(&mut self, descriptor: &MeshDescriptor, task: Task<Mesh>, parent: Entity) -> Entity;
    fn translate_entity(&mut self, entity: Entity, transform: Vec3);
}

impl<'w, 's> CommandExt for Commands<'w, 's> {
    fn spawn_chunk_display(&mut self, descriptor: &MeshDescriptor, task: Task<Mesh>, parent: Entity) -> Entity {
        self.spawn(ChunkDisplayBundle::new(&descriptor, task)).set_parent(parent).id()
    }
    fn translate_entity(&mut self, entity: Entity, transform: Vec3) {
        self.entity(entity).insert(Transform::from_translation(transform));
    }
}
