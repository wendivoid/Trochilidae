use bevy_core_pipeline::{prelude::*, prepass::DepthPrepass};
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_pbr::NotShadowCaster;
use bevy_transform::prelude::*;

use super::WASDController;

#[derive(Bundle)]
pub struct ObserverBundle {
    pub observer: crate::observer::Observer,
    pub pancam: PanOrbitCamera,
    pub camera: Camera3dBundle,
    pub wasd_controller: WASDController,
    pub depth_prepass: DepthPrepass,
    pub shadow_castor: NotShadowCaster,
}

impl Default for ObserverBundle {
    fn default() -> ObserverBundle {
        ObserverBundle {
            pancam: PanOrbitCamera::default(),
            observer: Default::default(),
            depth_prepass: DepthPrepass,
            wasd_controller: WASDController::default(),
            camera: Camera3dBundle {
                transform: Transform::from_xyz(25.0, 25.0, 25.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            },
            shadow_castor: NotShadowCaster,
        }
    }
}
