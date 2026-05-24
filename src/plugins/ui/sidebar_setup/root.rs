use bevy::prelude::*;

use super::sidebar;

pub fn spawn_root(mut commands: Commands, images: &mut Assets<Image>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|root| {
            sidebar::spawn_sidebar(root, images);
        });
}
