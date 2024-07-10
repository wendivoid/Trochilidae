use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct ObserverDiagnostics {
    pub hex: hexx::Hex,
    pub chunk: hexx::Hex
}