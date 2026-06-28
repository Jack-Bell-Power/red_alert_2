use bevy::prelude::*;

use crate::plugins::{
    game::konstrua_metado::Metstato, ui::sidebar_setup::enums::icon::SidebarBuildButton,
};

pub fn button_click_system(
    mut metstato: ResMut<Metstato>,
    query: Query<(&Interaction, &SidebarBuildButton), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, button) in &query {
        if *interaction == Interaction::Pressed {
            match button {
                SidebarBuildButton::Gwepicon => {
                    metstato.aktiva = true;
                }
            }
        }
    }
}
