use bevy_ecs::prelude::*;
use bevy_time::{Timer, TimerMode};
use derive_more::{Deref, DerefMut};

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct CycleTimer(pub Timer);

impl Default for CycleTimer {
    fn default() -> CycleTimer {
        CycleTimer(Timer::new(
            bevy_utils::Duration::from_millis(1000),
            TimerMode::Repeating,
        ))
    }
}
