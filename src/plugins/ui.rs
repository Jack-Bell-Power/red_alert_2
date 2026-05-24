use bevy::prelude::*;

mod sidebar_setup;

pub struct Ui;

impl Plugin for Ui {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sidebar_setup::setup_sidebar);
    }
}
