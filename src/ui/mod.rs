use bevy::prelude::*;

use crate::AppState;

mod menu;

#[derive(Debug)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), menu::setup)
            .add_systems(Update, menu::handle.run_if(in_state(AppState::Menu)))
            .add_systems(OnExit(AppState::Menu), menu::cleanup);
    }
}
