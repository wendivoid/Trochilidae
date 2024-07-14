use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_state::condition::in_state;
use bevy_state::state::States;

use super::systems::*;

use crate::observer::systems::spawn_viewport_assembly;

pub struct SkyPlugin<S: ScheduleLabel + Clone, U: States + Clone> {
    spawn: S,
    update: U,
}

impl<S: ScheduleLabel + Clone, U: States + Clone> Plugin for SkyPlugin<S, U> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cycle.run_if(in_state(self.update.clone())));
        app.add_systems(self.spawn.clone(), spawn.after(spawn_viewport_assembly));
    }
}

impl<S: ScheduleLabel + Clone, U: States + Clone> SkyPlugin<S, U> {
    pub fn new(spawn: S, update: U) -> SkyPlugin<S, U> {
        SkyPlugin { spawn, update }
    }
}
