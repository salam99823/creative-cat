use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use builder::MapBuilder;

use crate::settings::LEVEL_SIZE;

use super::MapTile;

mod builder;

const NUM_TILES: usize = (LEVEL_SIZE.x * LEVEL_SIZE.y) as usize;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Wall,
    Floor,
    Void,
}

#[derive(Debug)]
pub struct Map {
    pub tiles: Vec<TileType>,

    pub occupation: Vec<Option<Entity>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Void; NUM_TILES],
            occupation: vec![None; NUM_TILES],
        }
    }

    pub fn in_bounds<T: Into<UVec2>>(&self, position: T) -> bool {
        let position: UVec2 = position.into();
        position.x >= 0 && position.x < LEVEL_SIZE.x && position.y >= 0 && position.y < LEVEL_SIZE.y
    }

    pub fn can_enter_tile<T: Into<UVec2>>(&self, position: T) -> bool {
        let position = position.into();
        self.in_bounds(position) && self.tiles[map_idx(position.x, position.y)] == TileType::Floor
    }

    pub fn is_tile_occupied<T: Into<UVec2>>(&self, position: T) -> bool {
        let position = position.into();
        self.in_bounds(position) && self.occupation[map_idx(position.x, position.y)] == None
    }

    pub fn try_idx(&self, position: UVec2) -> Option<usize> {
        if !self.in_bounds(position) {
            None
        } else {
            Some(map_idx(position.x, position.y))
        }
    }
}

pub fn map_idx(x: u32, y: u32) -> usize {
    ((y * LEVEL_SIZE.x) + x) as usize
}

pub fn spawn_map_tiles(mut commands: Commands, mb: Res<MapBuilder>) {
    for y in 0..LEVEL_SIZE.y {
        for x in 0..LEVEL_SIZE.x {
            let idx = map_idx(x, y);

            match mb.map.tiles[idx] {
                TileType::Floor => {
                    commands
                        .spawn((
                            Sprite {
                                color: Color::srgba(0.529, 0.529, 0.529, 1.0),
                                custom_size: Some(Vec2::new(1.0, 1.0)),
                                ..Default::default()
                            },
                            Visibility::Visible,
                        ))
                        .insert(MapTile)
                        .insert(TilePos::new(x, y))
                        .insert(TilemapSize::new(32, 32));
                }
                TileType::Wall => {
                    commands
                        .spawn((
                            atlas.atlas.clone(),
                            Sprite {
                                color: Color::srgba(0.301, 0.301, 0.301, 1.0),
                                custom_size: Some(Vec2::new(1.0, 1.0)),
                                ..Default::default()
                            },
                            ..Default::default(),
                        ))
                        .insert(MapTile)
                        .insert(TilePos::new(x, y))
                        .insert(TilemapSize::new(32, 32));
                }
                TileType::Void => (),
            }
        }
    }
}
