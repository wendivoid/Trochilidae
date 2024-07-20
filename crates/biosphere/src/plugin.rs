use bevy_app::prelude::*;
use bevy_ecs::schedule::{IntoSystemConfigs, ScheduleLabel};
use bevy_state::{condition::in_state, state::States};

use crate::systems::*;

pub struct BioSpherePlugin<S: ScheduleLabel + Clone, U: States + Clone> {
    spawn: S,
    update: U,
}

impl<S: ScheduleLabel + Clone, U: States + Clone> Plugin for BioSpherePlugin<S, U> {
    fn build(&self, app: &mut App) {
        app.add_plugins(plants::trees::render::VascularMaterialPlugin::default());
        app.add_systems(
            self.spawn.clone(),
            (
                spawn_biosphere.after(world::core::systems::spawn_simulation),
                genesis.after(spawn_biosphere),
            )
                .after(world::core::systems::spawn_simulation),
        );
        app.add_systems(
            Update,
            update_visible_instances.run_if(in_state(self.update.clone())),
        );
    }
}

impl<S: ScheduleLabel + Clone, U: States + Clone> BioSpherePlugin<S, U> {
    pub fn new(spawn: S, update: U) -> BioSpherePlugin<S, U> {
        BioSpherePlugin { spawn, update }
    }
}
