use bevy_app::prelude::*;
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_state::{condition::in_state, state::States};

use super::systems;

pub struct ObserverPlugin<S: ScheduleLabel + Clone, U: States + Clone> {
    spawn: S,
    update: U,
}

impl<S: ScheduleLabel + Clone, U: States + Clone> Plugin for ObserverPlugin<S, U> {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin)
            .add_systems(self.spawn.clone(), systems::spawn_viewport_assembly)
            .add_systems(
                Update,
                systems::update.run_if(in_state(self.update.clone())),
            );
    }
}

impl<S: ScheduleLabel + Clone, U: States + Clone> ObserverPlugin<S, U> {
    pub fn new(spawn: S, update: U) -> ObserverPlugin<S, U> {
        ObserverPlugin { spawn, update }
    }
}
