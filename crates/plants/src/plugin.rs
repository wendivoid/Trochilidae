use bevy_pbr::MaterialPlugin;
use bevy_app::prelude::*;

pub struct PlantPlugin;

impl Plugin for PlantPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<crate::material::PlantMaterial>::default());
    }
}