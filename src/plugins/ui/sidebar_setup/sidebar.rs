use bevy::prelude::*;

mod addon;
mod image_loader;
mod radar;
mod side1;
mod side2;
mod side3;
mod tabs;
mod top;

const SIDEBAR_PAL_PATH: &str = "assets/shp/ui/sidebar/sidebar.pal";

pub fn spawn_sidebar(root: &mut ChildSpawnerCommands, images: &mut Assets<Image>) {
    // Right sidebar.
    root.spawn((
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
        tabs::spawn_tabs(sidebar, images);
        top::spawn_top(sidebar, images);
        radar::spawn_radar(sidebar, images);
        side1::spawn_side1(sidebar, images);
        side2::spawn_side2(sidebar, images);
        side3::spawn_side3(sidebar, images);
        addon::spawn_addon(sidebar, images);
    });
}
