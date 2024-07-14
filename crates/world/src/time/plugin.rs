use bevy_app::prelude::*;
use bevy_time::{Time, Virtual};

use crate::sky::CycleTimer;

use super::TimeSettings;

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeSettings>()
            .insert_resource(CycleTimer::default())
            .insert_resource(Time::<Virtual>::default());
    }
}
