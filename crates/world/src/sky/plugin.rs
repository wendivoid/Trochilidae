use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;

pub struct SkyPlugin<U: ScheduleLabel + Clone, U2: ScheduleLabel + Clone> {
    spawn: U2,
    update: U,
}

impl<U: ScheduleLabel + Clone, U2: ScheduleLabel + Clone> Plugin for SkyPlugin<U, U2> {
    fn build(&self, app: &mut App) {
        app.add_systems(self.update.clone(), super::systems::cycle);
        app.add_systems(
            self.spawn.clone(),
            super::systems::spawn.after(crate::systems::spawn_viewport_assembly),
        );
    }
}

impl<U: ScheduleLabel + Clone, U2: ScheduleLabel + Clone> SkyPlugin<U, U2> {
    pub fn new(spawn: U2, update: U) -> SkyPlugin<U, U2> {
        SkyPlugin { spawn, update }
    }
}
