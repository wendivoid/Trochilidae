use bevy_app::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;

pub struct DebugPlugin<S: ScheduleLabel + Clone, U: ScheduleLabel + Clone> {
    spawn: S,
    update: U,
}

impl<S: ScheduleLabel + Clone, U: ScheduleLabel + Clone> Plugin for DebugPlugin<S, U> {
    fn build(&self, app: &mut App) {
        app.add_systems(self.spawn.clone(), super::systems::spawn);
        app.add_systems(
            self.update.clone(),
            (super::systems::update_time, super::systems::update_observer),
        );
    }
}

impl<U: ScheduleLabel + Clone, S: ScheduleLabel + Clone> DebugPlugin<S, U> {
    pub fn new(spawn: S, update: U) -> DebugPlugin<S, U> {
        DebugPlugin { spawn, update }
    }
}
