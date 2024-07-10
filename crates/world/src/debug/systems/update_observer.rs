use bevy_ecs::prelude::*;
use bevy_text::prelude::*;

use crate::{core::ObserverDiagnostics, debug::ObserverText};

pub fn update_observer(
    observer: Res<ObserverDiagnostics>,
    mut observer_hex: Query<(&mut Text, &ObserverText), With<ObserverText>>,
) {
    if observer.is_changed() {
        for (mut text, ob) in observer_hex.iter_mut() {
            match ob {
                ObserverText::Chunk => {
                    text.sections[1].value = format!("({}, {})", observer.chunk.x, observer.chunk.y)
                },
                ObserverText::Hex => {
                    text.sections[1].value = format!("({}, {})", observer.hex.x, observer.hex.y);
                }
            }
        }
    }
}