use bevy::prelude::*;

#[derive(Component, Debug)]
pub enum SidebarBuildButton {
    Gwepicon {
        width: u16,
        height: u16,
    },
}
