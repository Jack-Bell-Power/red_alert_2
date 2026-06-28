use bevy::prelude::*;

pub mod konstrua_metado;
mod spawn_sprite;

pub struct Systems;

impl Plugin for Systems {
    fn build(&self, app: &mut App) {
        app.init_resource::<konstrua_metado::Metstato>();
        app.add_systems(Startup, spawn_sprite::setup)
            .add_systems(Update, spawn_sprite::animate_sprite)
            .add_systems(Update, konstrua_metado::metada_sistemo)
            .add_systems(Update, konstrua_metado::ĝisdatigi_antaŭvidan_lokigan_sistemon);
    }
}
