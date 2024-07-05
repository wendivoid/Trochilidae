use bevy::prelude::*;
use sickle_ui::{prelude::*, SickleUiPlugin};

pub struct ExampleUiPlugin;

impl Plugin for ExampleUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SickleUiPlugin);
        app.add_systems(Startup, build_ui);
    }
}
fn build_ui(mut commands: Commands) {
    let systems = vec!["Monopodial Tree", "Sympodial Tree", "Ternary Tree"];
    commands.ui_builder(UiRoot).column(|column| {
        column
            .style()
            .height(Val::Auto)
            .margin(UiRect::all(Val::Px(15.0)))
            .padding(UiRect::all(Val::Px(5.0)))
            .background_color(Color::rgba(0.0, 0.0, 0.0, 5.0));
        column.row(|row| {
            row.checkbox(Some("Wireframe".into()), false);
        });
        column.row(|row| {
            row.dropdown(systems, Some(0));
        });
    });
}