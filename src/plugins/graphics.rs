use bevy::prelude::*;

mod camera2d;
mod mesh;

pub struct Graphics;

impl Plugin for Graphics {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (mesh::setup, camera2d::setup_camera2d));
    }
}
