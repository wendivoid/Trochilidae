use bevy_ecs::prelude::*;
use bevy_text::prelude::*;
use bevy_time::{Time, Virtual};

use crate::{core::debug::TimeText, time::TimeSettings};

pub fn update_time(
    time: Res<Time<Virtual>>,
    settings: Res<TimeSettings>,
    mut observer_hex: Query<(&mut Text, &TimeText), With<TimeText>>,
) {
    if time.is_changed() {
        for (mut text, ob) in observer_hex.iter_mut() {
            match ob {
                TimeText::DateTime => {
                    let elapsed = time.elapsed_seconds();
                    let seconds_per_day = settings.seconds_per_hour *settings.hours_per_day;
                    let seconds_per_year = seconds_per_day as u16 * settings.days_per_year;
                    text.sections[1].value = format!("{}", elapsed as u32 % seconds_per_day as u32);
                    text.sections[3].value = format!("{:.0}", elapsed as u32 / seconds_per_day as u32);
                    text.sections[5].value = format!("{:.0}", elapsed / seconds_per_year as f32);
                },
                TimeText::Time => {
                    text.sections[1].value = format!("{:.0}", time.elapsed_seconds());
                }
            }
        }
    }
}