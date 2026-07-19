use bevy::prelude::*;
use std::path::Path;

use super::image_loader;
use crate::plugins::ui::sidebar_setup::{
    enums::icon::SidebarBuildButton, sidebar::SIDEBAR_PAL_PATH,
};

pub fn spawn_side2(sidebar: &mut ChildSpawnerCommands, images: &mut Assets<Image>) {
    if let Some(handle) = image_loader::load_image(
        images,
        Path::new("assets/shp/ui/sidebar/side2.shp"),
        Path::new(SIDEBAR_PAL_PATH),
    ) {
        let mut i = 0;
        while i < 9 {
            let mut entity = sidebar.spawn((
                ImageNode::new(handle.clone()),
                Node {
                    width: Val::Px(168.0),
                    height: Val::Px(50.0),
                    ..default()
                },
            ));

            if i == 0 {
                entity.with_children(|side2| {
                    side2
                        .spawn((
                            Button,
                            SidebarBuildButton::Gwepicon {
                                width: 3,
                                height: 5,
                            },
                            Node {
                                width: Val::Px(60.0),
                                height: Val::Px(48.0),
                                top: Val::Px(2.0),
                                left: Val::Px(22.0),
                                position_type: PositionType::Absolute,
                                ..default()
                            },
                        ))
                        .with_children(|button| {
                            if let Some(handle) = image_loader::load_image(
                                images,
                                Path::new("assets/shp/ui/icon/gwepicon.shp"),
                                Path::new("assets/shp/ui/icon/cameo.pal"),
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
            i += 1;
        }
    }
}
