use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_transform::prelude::*;
use bevy_core_pipeline::prelude::*;

#[derive(Bundle)]
pub struct ObserverBundle {
    pub observer: crate::components::Observer,
    pub camera: Camera3dBundle,
    pub pancam: PanOrbitCamera
}

impl Default for ObserverBundle {
    fn default() -> ObserverBundle {
        ObserverBundle {
            pancam: PanOrbitCamera::default(),
            observer: Default::default(),
            camera: Camera3dBundle {
                transform: Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            }
        }
    }
}
