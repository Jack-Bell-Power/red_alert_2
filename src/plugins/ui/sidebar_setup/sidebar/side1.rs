use bevy::prelude::*;
use std::path::Path;

use super::image_loader;
use crate::plugins::ui::sidebar_setup::sidebar::SIDEBAR_PAL_PATH;

pub fn spawn_side1(sidebar: &mut ChildSpawnerCommands, images: &mut Assets<Image>) {
    if let Some(handle) = image_loader::load_image(
        images,
        Path::new("assets/shp/ui/sidebar/side1.shp"),
        Path::new(SIDEBAR_PAL_PATH),
    ) {
        sidebar
            .spawn((
                ImageNode::new(handle),
                Node {
                    width: Val::Px(168.0),
                    height: Val::Px(69.0),
                    ..default()
                },
            ))
            .with_children(|side1| {
                //Repair.
                side1
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(52.0),
                            height: Val::Px(32.0),
                            top: Val::Px(8.0),
                            left: Val::Px(33.0),
                            ..default()
                        },
                    ))
                    .with_children(|button| {
                        if let Some(handle) = image_loader::load_image(
                            images,
                            Path::new("assets/shp/ui/sidebar/repair.shp"),
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
                //Sell.
                side1
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(52.0),
                            height: Val::Px(32.0),
                            top: Val::Px(8.0),
                            left: Val::Px(33.0),
                            ..default()
                        },
                    ))
                    .with_children(|button| {
                        if let Some(handle) = image_loader::load_image(
                            images,
                            Path::new("assets/shp/ui/sidebar/sell.shp"),
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
                //Tab00.
                side1
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(32.0),
                            height: Val::Px(28.0),
                            top: Val::Px(39.0),
                            left: Val::Px(20.0),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                    ))
                    .with_children(|button| {
                        if let Some(handle) = image_loader::load_image(
                            images,
                            Path::new("assets/shp/ui/sidebar/tab00.shp"),
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
                //Tab01.
                side1
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(32.0),
                            height: Val::Px(28.0),
                            top: Val::Px(39.0),
                            left: Val::Px(52.0),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                    ))
                    .with_children(|button| {
                        if let Some(handle) = image_loader::load_image(
                            images,
                            Path::new("assets/shp/ui/sidebar/tab01.shp"),
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
                //Tab02.
                side1
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(32.0),
                            height: Val::Px(28.0),
                            top: Val::Px(39.0),
                            left: Val::Px(84.0),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                    ))
                    .with_children(|button| {
                        if let Some(handle) = image_loader::load_image(
                            images,
                            Path::new("assets/shp/ui/sidebar/tab02.shp"),
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
                //Tab03.
                side1
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(32.0),
                            height: Val::Px(28.0),
                            top: Val::Px(39.0),
                            left: Val::Px(116.0),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                    ))
                    .with_children(|button| {
                        if let Some(handle) = image_loader::load_image(
                            images,
                            Path::new("assets/shp/ui/sidebar/tab03.shp"),
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
