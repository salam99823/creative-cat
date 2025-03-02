use assets::PlayerLayouts;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{AppState, camera::Target};
use mods::cat;

mod assets;
mod mods;

#[derive(Debug, Clone, Copy, Component)]
pub struct Player;

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AppState = AppState::InGame)]
pub enum PlayerState {
    #[default]
    Cat,
}

#[derive(Debug, Clone)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<PlayerState>()
            .add_systems(
                OnEnter(AppState::InGame),
                (assets::setup_clips, spawn_player).chain(),
            )
            .add_systems(OnEnter(PlayerState::Cat), cat::setup)
            .add_systems(Update, cat::movement.run_if(in_state(PlayerState::Cat)));
    }
}

fn spawn_player(
    mut commands: Commands,
    player_layouts: Res<PlayerLayouts>,
    assets: Res<AssetServer>,
) {
    commands.spawn((
        Target,
        Player,
        RigidBody::KinematicPositionBased,
        Collider::capsule_x(5., 10.),
        KinematicCharacterController::default(),
        Sprite::from_atlas_image(
            assets.load("PlayerCat.png"),
            TextureAtlas {
                layout: player_layouts.cat.clone_weak(),
                ..default()
            },
        ),
    ));
}
