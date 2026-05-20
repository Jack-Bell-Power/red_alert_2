use bevy::prelude::*;

pub fn setup_camera2d(mut commands: Commands) {
    commands.spawn(Camera2d);
}
