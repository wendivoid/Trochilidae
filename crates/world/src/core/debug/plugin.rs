use bevy_app::prelude::*;
use bevy_ecs::schedule::{IntoSystemConfigs, ScheduleLabel};
use bevy_state::{condition::in_state, state::States};

pub struct DebugPlugin<S: ScheduleLabel + Clone, U: States + Clone> {
    spawn: S,
    update: U,
}

impl<S: ScheduleLabel + Clone, U: States + Clone> Plugin for DebugPlugin<S, U> {
    fn build(&self, app: &mut App) {
        app.add_systems(self.spawn.clone(), super::systems::spawn);
        app.add_systems(
            Update,
            (super::systems::update_time, super::systems::update_observer)
                .run_if(in_state(self.update.clone())),
        );
    }
}

impl<S: ScheduleLabel + Clone, U: States + Clone> DebugPlugin<S, U> {
    pub fn new(spawn: S, update: U) -> DebugPlugin<S, U> {
        DebugPlugin { spawn, update }
    }
}
