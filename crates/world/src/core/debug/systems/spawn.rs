use bevy_ui::prelude::*;
use bevy_ecs::prelude::*;
use bevy_core::prelude::*;
use bevy_text::prelude::*;
use bevy_color::prelude::*;
use bevy_hierarchy::prelude::*;

use crate::{core::debug::*, WorldSettings};

pub fn spawn(
    mut commands: Commands,
    settings: Res<WorldSettings>
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_self: AlignSelf::End,
                margin: UiRect::axes(Val::Px(5.0), Val::Px(5.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Name::new("Debug UI")
    )).with_children(|commands| {
        boxed(commands, |cmds| {
            cmds.spawn(TextBundle::from_section("World", big_text()));
            cmds.spawn(world_hex_text(&settings));
            cmds.spawn(world_chunk_text(&settings));
        });
        boxed(commands, |cmds| {
            cmds.spawn(TextBundle::from_section("Observer", big_text()));
            cmds.spawn(observer_hex_text()).insert(ObserverText::Hex);
            cmds.spawn(observer_chunk_text()).insert(ObserverText::Chunk);
        });
        boxed(commands, |cmds| {
            cmds.spawn(TextBundle::from_section("Time", big_text()));
            cmds.spawn(time_datetime_text()).insert(TimeText::DateTime);
            cmds.spawn(time_time_text()).insert(TimeText::Time);
        });
    });
}

fn boxed<F: Fn(&mut ChildBuilder)>(commands: &mut ChildBuilder, spawn: F) {
    commands.spawn(NodeBundle {
        style: Style {
            border: UiRect::all(Val::Px(1.0)),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(4.0)),
            margin: UiRect::all(Val::Px(2.0)),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
        border_color: Color::WHITE.into(),
        border_radius: BorderRadius::all(Val::Px(7.0).into()),
        ..Default::default()
    }).with_children(|commands| spawn(commands));
}

fn big_text() -> TextStyle {
    TextStyle {
        font_size: 16.0,
        ..Default::default()
    }
}

fn small_text() -> TextStyle {
    TextStyle {
        font_size: 12.0,
        ..Default::default()
    }
}

fn observer_hex_text() -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![
                TextSection::new("Hex: ", small_text()),
                TextSection::new("!!!!", small_text())
            ],
            ..Default::default()
        },
        ..Default::default()
    }
}

fn observer_chunk_text() -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![
                TextSection::new("Chunk: ", small_text()),
                TextSection::new("!!!!", small_text())
            ],
            ..Default::default()
        },
        ..Default::default()
    }
}

fn world_chunk_text(settings: &WorldSettings) -> TextBundle {
    let chunk_count = settings.all_chunks().count();
    TextBundle {
        text: Text {
            sections: vec![
                TextSection::new("Chunk: ", small_text()),
                TextSection::new(format!("{chunk_count}"), small_text())
            ],
            ..Default::default()
        },
        ..Default::default()
    }
}

fn world_hex_text(settings: &WorldSettings) -> TextBundle {
    let hex_count = settings.all_coords().count();
    TextBundle {
        text: Text {
            sections: vec![
                TextSection::new("Hex: ", small_text()),
                TextSection::new(format!("{hex_count}"), small_text())
            ],
            ..Default::default()
        },
        ..Default::default()
    }
}

fn time_datetime_text() -> TextBundle {
        TextBundle {
            text: Text {
                sections: vec![
                    TextSection::new("Date: ", small_text()),
                    TextSection::new("~", small_text()),
                    TextSection::new(" ", small_text()),
                    TextSection::new("~", small_text()),
                    TextSection::new("/", small_text()),
                    TextSection::new("~", small_text()),
                ],
                ..Default::default()
            },
            ..Default::default()
        }
}

fn time_time_text() -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![
                TextSection::new("Total: ", small_text()),
                TextSection::new("!!!!", small_text())
            ],
            ..Default::default()
        },
        ..Default::default()
    }
}