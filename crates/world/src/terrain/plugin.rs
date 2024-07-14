use bevy_state::{condition::in_state, state::States};
use bevy_app::prelude::*;
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};

use super::systems::*;

pub struct TerrainPlugin<S: ScheduleLabel + Clone, U: States + Clone> {
    spawn: S,
    update: U,
}

impl<S: ScheduleLabel + Clone, U: States + Clone> Plugin for TerrainPlugin<S, U> {
    fn build(&self, app: &mut App) {
        app.add_systems(self.spawn.clone(), spawn_material)
            .add_systems(
                Update,
                (spawn_terrain, check_terrain_tasks.after(spawn_terrain))
                            .run_if(in_state(self.update.clone())),
            );
    }
}

impl<S: ScheduleLabel + Clone, U: States + Clone> TerrainPlugin<S, U> {
    pub fn new(spawn: S, update: U) -> TerrainPlugin<S, U> {
        TerrainPlugin { spawn, update }
    }
}