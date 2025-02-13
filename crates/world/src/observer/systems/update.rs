use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::{
    observer::{Observer, WorldOrigin},
    WorldSettings,
};

pub fn update(
    settings: Res<WorldSettings>,
    mut origin: ResMut<WorldOrigin>,
    mut observer: Query<&mut PanOrbitCamera, (With<Observer>, Changed<PanOrbitCamera>)>,
) {
    for mut observer_camera in observer.iter_mut() {
        let layout = settings.layout();
        let bounds = settings.bounds();
        let observer_pos = observer_camera.target_focus.xz();
        let observer_hex = layout.world_pos_to_hex(observer_pos);
        let wrapped_observer_hex = bounds.wrap(observer_hex);
        let observer_chunk = wrapped_observer_hex.to_lower_res(settings.chunk_radius);
        if observer_hex != wrapped_observer_hex {
            let fract_pos = observer_pos - layout.hex_to_world_pos(observer_hex);
            let wrapped_position = fract_pos + layout.hex_to_world_pos(wrapped_observer_hex);
            observer_camera.target_focus = Vec3::new(
                wrapped_position.x,
                0.0,
                wrapped_position.y,
            );
            observer_camera.focus = Vec3::new(
                wrapped_position.x,
                0.0,
                wrapped_position.y,
            );
        }
        if origin.hex != Some(wrapped_observer_hex) {
            origin.hex = Some(wrapped_observer_hex);
        }
        if origin.chunk != Some(observer_chunk) {
            origin.chunk = Some(observer_chunk);
        }
    }
}
