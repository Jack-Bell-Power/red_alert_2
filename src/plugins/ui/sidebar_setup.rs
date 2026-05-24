use bevy::prelude::*;

mod root;
mod sidebar;

pub fn setup_sidebar(commands: Commands, mut images: ResMut<Assets<Image>>) {
    root::spawn_root(commands, &mut images);
}
