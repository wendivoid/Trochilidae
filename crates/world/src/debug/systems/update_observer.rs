use bevy_ecs::prelude::*;
use bevy_text::prelude::*;

use crate::{debug::ObserverText, WorldOrigin};

pub fn update_observer(
    observer: Res<WorldOrigin>,
    mut observer_hex: Query<(&mut Text, &ObserverText), With<ObserverText>>,
) {
    if observer.is_changed() {
        for (mut text, ob) in observer_hex.iter_mut() {
            match ob {
                ObserverText::Chunk => {
                    if let Some(chunk) = observer.chunk {
                        text.sections[1].value = format!("({}, {})", chunk.x, chunk.y)
                    } else {
                        text.sections[1].value = format!("None");
                    }
                },
                ObserverText::Hex => {
                    if let Some(hex) = observer.hex {
                        text.sections[1].value = format!("({}, {})", hex.x, hex.y);
                    } else {
                        text.sections[1].value = format!("None");
                    }
                }
            }
        }
    }
}