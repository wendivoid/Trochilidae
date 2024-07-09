use bevy_app::prelude::*;
use bevy_time::{Time, Timer, TimerMode, Virtual};

use crate::sky::CycleTimer;

use super::TimeSettings;

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeSettings>();
        app.insert_resource(CycleTimer(Timer::new(
            bevy_utils::Duration::from_millis(50),
            TimerMode::Repeating,
        )));
        app.insert_resource(Time::<Virtual>::default());
    }
}