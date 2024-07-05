use bevy_ecs::prelude::*;
use bevy_asset::prelude::*;
use bevy_render::prelude::*;
use bevy_transform::prelude::*;

use crate::material::PlantMaterial;

#[derive(Bundle)]
pub struct PlantBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<PlantMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl Default for PlantBundle {
    fn default() -> PlantBundle {
        PlantBundle {
            mesh: Default::default(),
            material: Default::default(),
            transform: Transform::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default()
        }
    }
}