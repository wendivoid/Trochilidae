use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_render::alpha::AlphaMode;

use crate::core::WorldOrigin;
use crate::sky::SkyPlugin;
use crate::time::TimePlugin;
use crate::{systems::*, WorldSettings};

pub struct WorldPlugin<S: ScheduleLabel + Clone = Update, S2: ScheduleLabel + Clone = Startup> {
    update: S,
    spawn: S2,
}

impl Default for WorldPlugin {
    fn default() -> WorldPlugin {
        WorldPlugin {
            update: Update,
            spawn: Startup,
        }
    }
}

impl<S: ScheduleLabel + Clone, S2: ScheduleLabel + Clone> Plugin for WorldPlugin<S, S2> {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldSettings>();
        app.init_resource::<WorldOrigin>();
        app.add_plugins(TimePlugin);
        app.insert_resource(bevy_water::WaterSettings {
            height: 0.1,
            spawn_tiles: None,
            amplitude: 0.0,
            clarity: 1.5,
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        });
        app.add_plugins(bevy_water::WaterPlugin);
        app.add_plugins(bevy_easings::EasingsPlugin);
        app.add_plugins(SkyPlugin::new(self.spawn.clone(), self.update.clone()));
        app.add_plugins(crate::debug::DebugPlugin::new(
            self.spawn.clone(),
            self.update.clone(),
        ));
        app.add_plugins(PanOrbitCameraPlugin);
        app.add_systems(
            self.update.clone(),
            (
                update_observer,
                update_mesh_tasks.after(update_observer),
                (check_water_tasks, check_terrain_tasks, check_moisture_tasks).after(update_mesh_tasks),
            ),
        );
        app.add_systems(
            self.spawn.clone(),
            (
                spawn_materials,
                spawn_viewport_assembly,
                spawn_simulation_world.after(spawn_viewport_assembly),
            ),
        );
    }
}
