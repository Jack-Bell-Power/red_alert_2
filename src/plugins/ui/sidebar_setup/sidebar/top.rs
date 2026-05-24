use bevy::prelude::*;
use std::path::Path;

use super::image_loader;
use crate::plugins::ui::sidebar_setup::sidebar::SIDEBAR_PAL_PATH;

pub fn spawn_top(sidebar: &mut ChildSpawnerCommands, images: &mut Assets<Image>) {
    if let Some(handle) = image_loader::load_image(
        images,
        Path::new("assets/shp/ui/sidebar/top.shp"),
        Path::new(SIDEBAR_PAL_PATH),
    ) {
        sidebar
            .spawn((
                ImageNode::new(handle),
                Node {
                    width: Val::Px(168.0),
                    height: Val::Px(32.0),
                    ..default()
                },
            ))
            .with_children(|top| {
                top.spawn((
                    Button,
                    Node {
                        width: Val::Px(72.0),
                        height: Val::Px(22.0),
                        top: Val::Px(5.0),
                        left: Val::Px(13.0),
                        ..default()
                    },
                ))
                .with_children(|button| {
                    if let Some(handle) = image_loader::load_image(
                        images,
                        Path::new("assets/shp/ui/sidebar/diplobtn.shp"),
                        Path::new(SIDEBAR_PAL_PATH),
                    ) {
                        button.spawn((
                            ImageNode::new(handle),
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                        ));
                    }
                });
                top.spawn((
                    Button,
                    Node {
                        width: Val::Px(72.0),
                        height: Val::Px(22.0),
                        top: Val::Px(5.0),
                        left: Val::Px(13.0),
                        ..default()
                    },
                ))
                .with_children(|button| {
                    if let Some(handle) = image_loader::load_image(
                        images,
                        Path::new("assets/shp/ui/sidebar/optbtn.shp"),
                        Path::new(SIDEBAR_PAL_PATH),
                    ) {
                        button.spawn((
                            ImageNode::new(handle),
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                        ));
                    }
                });
            });
    }
}
