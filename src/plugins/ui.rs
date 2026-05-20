use bevy::prelude::*;

mod sidebar;

pub struct Ui;

impl Plugin for Ui {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sidebar::setup_sidebar);
    }
}
