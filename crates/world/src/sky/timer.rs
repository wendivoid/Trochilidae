use bevy_ecs::prelude::*;
use bevy_time::Timer;

#[derive(Resource)]
pub struct CycleTimer(pub Timer);