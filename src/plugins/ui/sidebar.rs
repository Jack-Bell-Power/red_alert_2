use std::path::Path;

use bevy::prelude::*;

use crate::plugins::shp::shp_reader;

pub fn setup_sidebar(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // Fullscreen UI root
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            // Right sidebar
            parent
                .spawn((
                    Node {
                        width: Val::Px(168.0),
                        height: Val::Percent(100.0),

                        position_type: PositionType::Absolute,
                        right: Val::Px(0.0),

                        flex_direction: FlexDirection::Column,

                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                ))
                .with_children(|sidebar| {
                    //Sidebar image
                    //Tabs.
                    if let Some(mut entity) = spawn_image(sidebar, &mut images, "credits", 16.0) {
                        entity.with_children(|tab| {
                            tab.spawn((
                                Button,
                                Node {
                                    width: Val::Px(60.0),
                                    height: Val::Px(48.0),
                                    ..default()
                                },
                            ))
                            .with_children(|button| {
                                //Diplobtn.
                                if let Ok(handles) = shp_reader::decode_shp_to_image(
                                    &mut images,
                                    Path::new(&format!("assets/shp/ui/sidebar/diplobtn.shp")),
                                    Path::new("assets/shp/ui/sidebar/sidebar.pal"),
                                    false,
                                ) {
                                    if let Some(handle) = handles.get(0) {
                                        button.spawn((
                                            ImageNode::new(handle.clone()),
                                            Node {
                                                width: Val::Px(72.0),
                                                height: Val::Px(22.0),

                                                position_type: PositionType::Absolute,
                                                ..default()
                                            },
                                        ));
                                    }
                                }
                            });
                        });
                    }
                    //Top.
                    spawn_image(sidebar, &mut images, "top", 32.0);
                    //Radar.
                    spawn_image(sidebar, &mut images, "radar", 110.0);
                    //Side1.
                    spawn_image(sidebar, &mut images, "side1", 69.0);
                    let mut i = 0;
                    while i < 10 {
                        //Side2.
                        spawn_image(sidebar, &mut images, "side2", 50.0);
                        i += 1;
                    }
                    //Side3.
                    spawn_image(sidebar, &mut images, "side3", 26.0);
                    //addon.
                    spawn_image(sidebar, &mut images, "addon", 63.0);
                });
        });
}

fn spawn_image<'a>(
    entity: &'a mut ChildSpawnerCommands,
    images: &mut Assets<Image>,
    file_name: &str,
    height: f32,
) -> Option<EntityCommands<'a>> {
    if let Ok(handles) = shp_reader::decode_shp_to_image(
        images,
        Path::new(&format!("assets/shp/ui/sidebar/{}.shp", file_name)),
        Path::new("assets/shp/ui/sidebar/sidebar.pal"),
        false,
    ) {
        if let Some(handle) = handles.get(0) {
            let entity = entity.spawn((
                ImageNode::new(handle.clone()),
                Node {
                    width: Val::Px(168.0),
                    height: Val::Px(height),
                    ..default()
                },
            ));

            return Some(entity);
        }
    }

    None
}
