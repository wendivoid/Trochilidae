use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_time::prelude::*;
use bevy_pbr::prelude::*;
use bevy_transform::prelude::*;

use bevy_pbr::light_consts::lux::AMBIENT_DAYLIGHT;

use crate::{sky::{Sun, CycleTimer}, time::TimeSettings};

pub fn cycle(
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut timer: ResMut<CycleTimer>,
    time: Res<Time<Virtual>>,
    time_settings: Res<TimeSettings>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        let t = time.elapsed_seconds()
            / ((time_settings.hours_per_day as f32 * time_settings.seconds_per_hour as f32) / 2.0);

        if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
            light_trans.rotation = Quat::from_rotation_x(-t);
            directional.illuminance = t.sin().max(0.0).powf(2.0) * AMBIENT_DAYLIGHT;
        }
    }
}