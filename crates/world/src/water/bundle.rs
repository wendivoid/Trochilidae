use bevy_asset::Handle;
use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_pbr::NotShadowCaster;
use bevy_render::prelude::*;

use super::{StandardWaterMaterial, WATER_MATERIAL};

#[derive(Bundle)]
pub struct WaterBundle {
    pub name: Name,
    pub spatial: SpatialBundle,
    pub shadow_caster: NotShadowCaster,
    pub material_handle: Handle<StandardWaterMaterial>,
}

impl Default for WaterBundle {
    fn default() -> WaterBundle {
        WaterBundle {
            name: Name::new("Water"),
            shadow_caster: NotShadowCaster,
            spatial: Default::default(),
            material_handle: WATER_MATERIAL,
        }
    }
}
