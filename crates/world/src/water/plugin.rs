use bevy_app::prelude::*;
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
use bevy_pbr::MaterialPlugin;
use bevy_state::{condition::in_state, state::States};

use crate::core::systems::update_viewport;

use super::systems::*;

pub struct WaterPlugin<S: ScheduleLabel + Clone, U: States + Clone> {
    spawn: S,
    update: U,
}

impl<S: ScheduleLabel + Clone, U: States + Clone> Plugin for WaterPlugin<S, U> {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<super::StandardWaterMaterial>::default());
        app.add_systems(self.spawn.clone(), spawn_material)
            .add_systems(
                Update,
                (
                    spawn_water.after(update_viewport),
                    check_water_tasks.after(spawn_water),
                )
                    .run_if(in_state(self.update.clone())),
            );
    }
}

impl<S: ScheduleLabel + Clone, U: States + Clone> WaterPlugin<S, U> {
    pub fn new(spawn: S, update: U) -> WaterPlugin<S, U> {
        WaterPlugin { spawn, update }
    }
}
