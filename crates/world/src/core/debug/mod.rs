mod plugin;
mod systems;

pub use self::plugin::DebugPlugin;

#[derive(bevy_ecs::prelude::Component)]
pub enum ObserverText {
    Chunk,
    Hex,
}

#[derive(bevy_ecs::prelude::Component)]
pub enum TimeText {
    DateTime,
    Time,
}
