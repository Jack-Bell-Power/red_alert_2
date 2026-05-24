use bevy::prelude::*;
use std::path::Path;

use super::image_loader;
use crate::plugins::ui::sidebar_setup::sidebar::SIDEBAR_PAL_PATH;

pub fn spawn_side3(sidebar: &mut ChildSpawnerCommands, images: &mut Assets<Image>) {
    if let Some(handle) = image_loader::load_image(
        images,
        Path::new("assets/shp/ui/sidebar/side3.shp"),
        Path::new(SIDEBAR_PAL_PATH),
    ) {
        sidebar.spawn((
            ImageNode::new(handle),
            Node {
                width: Val::Px(168.0),
                height: Val::Px(26.0),
                ..default()
            },
        ));
    }
}
