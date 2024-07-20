use bevy_ecs::prelude::*;
use bevy_input::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_transform::components::Transform;

use crate::observer::WASDController;

pub fn keyboard_bindings(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut observer: Query<(&mut PanOrbitCamera, &WASDController, &Transform)>,
) {
    for (mut camera, settings, trans) in observer.iter_mut() {
        if keyboard.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            camera.target_focus += trans.left() * settings.scale;
            camera.target_focus.y = 0.0;
        } else if keyboard.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            camera.target_focus += trans.right() * settings.scale;
            camera.target_focus.y = 0.0;
        }
        if keyboard.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            camera.target_focus += trans.forward() * settings.scale;
            camera.target_focus.y = 0.0;
        } else if keyboard.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            camera.target_focus += trans.back() * settings.scale;
            camera.target_focus.y = 0.0;
        }
    }
}