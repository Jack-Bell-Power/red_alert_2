use bevy::prelude::*;

use crate::plugins::{
    game::building_placement::Placement, ui::sidebar_setup::enums::icon::SidebarBuildButton,
};

pub fn button_click_system(
    mut placement: ResMut<Placement>,
    query: Query<(&Interaction, &SidebarBuildButton), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, button) in &query {
        if *interaction == Interaction::Pressed {
            match button {
                SidebarBuildButton::Gwepicon => {
                    placement.active = true;
                }
            }
        }
    }
}
