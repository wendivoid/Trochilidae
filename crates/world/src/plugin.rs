use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

use crate::map::HexState;
use crate::systems;

pub struct WorldPlugin<
    S: ScheduleLabel + Clone = Update,
    S2: ScheduleLabel + Clone = Startup
> {
    update: S,
    spawn: S2,
}

impl Default for WorldPlugin {
    fn default() -> WorldPlugin {
        WorldPlugin {
            update: Update,
            spawn: Startup
        }
    }
}

impl<S: ScheduleLabel + Clone, S2: ScheduleLabel + Clone> Plugin for WorldPlugin<S, S2> {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::compose::CellComposer>();
        app.init_resource::<crate::WorldSettings>();
        app.init_resource::<HexState>();
        app.add_plugins(PanOrbitCameraPlugin);
        app.add_systems(
            self.update.clone(),
            (systems::update_map, systems::spawn_chunk),
        );
        app.add_systems(
            self.spawn.clone(),
            (systems::spawn_observer, systems::spawn_world.after(systems::spawn_observer)),
        );
    }
}
