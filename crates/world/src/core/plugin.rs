use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_state::condition::in_state;
use bevy_state::state::States;

use super::debug::DebugPlugin;
use super::systems::*;
use crate::moisture::MoisturePlugin;
use crate::observer::ObserverPlugin;
use crate::sky::SkyPlugin;
use crate::terrain::TerrainPlugin;
use crate::time::TimePlugin;
use crate::water::WaterPlugin;
use crate::WorldSettings;

pub struct WorldPlugin<S: ScheduleLabel + Clone, U: States + Clone> {
    update: U,
    spawn: S,
}

impl<S: ScheduleLabel + Clone, U: States + Clone> WorldPlugin<S, U> {
    pub fn new(spawn: S, update: U) -> WorldPlugin<S, U> {
        WorldPlugin { spawn, update }
    }
}

impl<S: ScheduleLabel + Clone, U: States + Clone> Plugin for WorldPlugin<S, U> {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldSettings>()
            .init_resource::<crate::observer::WorldOrigin>()
            .add_event::<crate::InsertWorldChunk>()
            .add_plugins(TimePlugin)
            .add_plugins(ObserverPlugin::new(self.spawn.clone(), self.update.clone()))
            .add_plugins(SkyPlugin::new(self.spawn.clone(), self.update.clone()))
            .add_plugins(TerrainPlugin::new(self.spawn.clone(), self.update.clone()))
            .add_plugins(WaterPlugin::new(self.spawn.clone(), self.update.clone()))
            .add_plugins(MoisturePlugin::new(self.spawn.clone(), self.update.clone()))
            .add_plugins(DebugPlugin::new(self.spawn.clone(), self.update.clone()));
        app.add_systems(
            Update,
            (update_viewport.after(crate::observer::systems::update),)
                .run_if(in_state(self.update.clone())),
        );
        app.add_systems(
            self.spawn.clone(),
            (spawn_simulation.after(crate::observer::systems::spawn_viewport_assembly),),
        );
    }
}
