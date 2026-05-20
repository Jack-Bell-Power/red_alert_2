use bevy::prelude::*;

mod errors;
mod game;
mod graphics;
mod map;
mod pal;
mod shp;
mod ui;
mod window_setup;

pub struct Plugins;

impl Plugin for Plugins {
    fn build(&self, app: &mut App) {
        //app.add_plugins(shp::Shp);
        app.add_plugins(window_setup::WindowSetup)
            .add_plugins(game::Systems)
            .add_plugins(graphics::Graphics)
            .add_plugins(ui::Ui);
    }
}
