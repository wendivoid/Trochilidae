use std::f32::consts::PI;

use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_pbr::prelude::*;
use bevy_time::prelude::*;
use bevy_transform::prelude::*;

use bevy_pbr::light_consts::lux::OVERCAST_DAY;
use light_consts::lux::MOONLESS_NIGHT;

use crate::{
    sky::{CycleTimer, Sun},
    time::TimeSettings,
};

pub fn cycle(
    time: Res<Time<Virtual>>,
    time_settings: Res<TimeSettings>,
    mut commands: Commands,
    mut timer: ResMut<CycleTimer>,
    mut query: Query<(Entity, &Transform, &mut DirectionalLight), With<Sun>>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        if let Some((entity, light_trans, mut directional)) = query.single_mut().into() {
            let t = time_settings.hours_per_day * time_settings.seconds_per_hour;
            let step = PI * 2.0 / t as f32;
            let steps_in_day = t as f32 * step;
            let mut new = light_trans.clone();
            new.rotate_around(Vec3::ZERO, Quat::from_rotation_z(step));
            commands.entity(entity).insert(new);
            directional.illuminance = OVERCAST_DAY.lerp(MOONLESS_NIGHT, (1.0 / steps_in_day) * step);
            directional.color = lerp_linear(
                Color::WHITE,
                bevy_color::Srgba::hex("#c3cde6").unwrap().into(),
                (1.0 / steps_in_day) * step,
            );
        }
    }
}

fn lerp_linear(color0: Color, color1: Color, t: f32) -> Color {
    let color0 = color0.to_linear();
    let color1 = color1.to_linear();
    let r0 = color0.red;
    let g0 = color0.green;
    let b0 = color0.blue;
    let r1 = color1.red;
    let g1 = color1.green;
    let b1 = color1.blue;

    Color::linear_rgb(r0 + t * (r1 - r0), g0 + t * (g1 - g0), b0 + t * (b1 - b0))
}
