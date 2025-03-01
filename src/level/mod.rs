use bevy::prelude::*;
mod map;

#[derive(Component, Debug, Default, Clone)]
pub struct MapTile;

#[derive(Debug)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}
