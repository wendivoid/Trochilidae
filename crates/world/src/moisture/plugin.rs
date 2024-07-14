use bevy_app::prelude::*;
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
use bevy_state::{condition::in_state, state::States};

use crate::core::systems::update_viewport;

use super::systems::*;

pub struct MoisturePlugin<S: ScheduleLabel + Clone, U: States + Clone> {
    spawn: S,
    update: U,
}

impl<S: ScheduleLabel + Clone, U: States + Clone> Plugin for MoisturePlugin<S, U> {
    fn build(&self, app: &mut App) {
        app.add_systems(self.spawn.clone(), spawn_material)
            .add_systems(
                Update,
                (
                    spawn_moisture.after(update_viewport),
                    check_moisture_tasks.after(spawn_moisture),
                )
                    .run_if(in_state(self.update.clone())),
            );
    }
}

impl<S: ScheduleLabel + Clone, U: States + Clone> MoisturePlugin<S, U> {
    pub fn new(spawn: S, update: U) -> MoisturePlugin<S, U> {
        MoisturePlugin { spawn, update }
    }
}
