use bevy::prelude::*;
use rand::Rng;

use crate::settings::LEVEL_SIZE;

use super::{Map, TileType, map_idx};

const NUM_ROOMS: usize = 20;

#[derive(Resource, Debug)]
pub struct MapBuilder {
    pub map: Map,
    walls: Vec<URect>,
    rooms: Vec<URect>,
    pub player_start: UVec2,
    pub enemies_start: Vec<UVec2>,
}

impl MapBuilder {
    pub fn new() -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            walls: Vec::new(),
            rooms: Vec::new(),
            player_start: UVec2 { x: 0, y: 0 },
            enemies_start: Vec::new(),
        };
        mb.fill(TileType::Void);
        mb.build_random_rooms();
        mb.build_corridors();

        mb.player_start = UVec2::from(mb.rooms[0].center());

        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self) {
        let mut rng = rand::rng();

        while self.rooms.len() < NUM_ROOMS {
            let p0 = UVec2::new(
                rng.random_range(2..LEVEL_SIZE.x - 12),
                rng.random_range(2..LEVEL_SIZE.y - 12),
            );
            let room = URect::from_corners(
                p0,
                p0 + UVec2::new(rng.random_range(2..12), rng.random_range(2..12)),
            );
            if !self.rooms.iter().any(|r| r.intersect(room).is_empty()) {
                let wall = URect::new(
                    room.min.x - 1,
                    room.min.y - 1,
                    room.max.x + 1,
                    room.max.y + 1,
                );
                for p in [room.min, room.max]
                    .iter()
                    .filter(|p| p.x > 0 && p.x < LEVEL_SIZE.x && p.y > 0 && p.y < LEVEL_SIZE.y)
                {
                    let idx = map_idx(p.x, p.y);
                    self.map.tiles[idx] = TileType::Floor;
                }

                for p in [wall.min, wall.max]
                    .iter()
                    .filter(|p| p.x > 0 && p.x < LEVEL_SIZE.x && p.y > 0 && p.y < LEVEL_SIZE.y)
                {
                    let idx = map_idx(p.x, p.y);
                    if self.map.tiles[idx] == TileType::Void {
                        self.map.tiles[idx] = TileType::Wall;
                    }
                }

                self.rooms.push(room);
                self.walls.push(wall);

                if self.rooms.len() > 1 {
                    self.enemies_start.push(UVec2::from(room.center()));
                }
            }
        }
    }

    fn apply_horizontal_tunnel_walls(&mut self, x1: u32, x2: u32, y: u32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(UVec2 { x, y }) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
            if let Some(idx) = self.map.try_idx(UVec2 { x, y: y - 1 }) {
                if self.map.tiles[idx as usize] == TileType::Void {
                    self.map.tiles[idx as usize] = TileType::Wall;
                }
            }
            if let Some(idx) = self.map.try_idx(UVec2 { x, y: y + 1 }) {
                if self.map.tiles[idx as usize] == TileType::Void {
                    self.map.tiles[idx as usize] = TileType::Wall;
                }
            }
        }
    }

    fn apply_vertical_tunnel_walls(&mut self, y1: u32, y2: u32, x: u32) {
        for y in y1.min(y2)..=y1.max(y2) {
            if let Some(idx) = self.map.try_idx(UVec2 { x, y }) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
            if let Some(idx) = self.map.try_idx(UVec2 { x: x - 1, y }) {
                if self.map.tiles[idx as usize] == TileType::Void {
                    self.map.tiles[idx as usize] = TileType::Wall;
                }
            }
            if let Some(idx) = self.map.try_idx(UVec2 { x: x + 1, y }) {
                if self.map.tiles[idx as usize] == TileType::Void {
                    self.map.tiles[idx as usize] = TileType::Wall;
                }
            }
        }
    }

    fn build_corridors(&mut self) {
        let mut rng = rand::rng();
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.random_range(0..2) == 1 {
                self.apply_horizontal_tunnel_walls(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel_walls(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel_walls(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel_walls(prev.x, new.x, new.y);
            }
        }
    }

    pub fn entity_occupy_tile(&mut self, entity: Entity, position: UVec2) {
        let idx = map_idx(position.x, position.y);
        self.map.occupation[idx] = Some(entity);
    }

    pub fn free_occupy_tile(&mut self, position: UVec2) {
        let idx = map_idx(position.x, position.y);
        self.map.occupation[idx] = None;
    }

    pub fn move_entity_occupation(&mut self, entity: Entity, old_p: UVec2, new_p: UVec2) {
        let old_idx = map_idx(old_p.x, old_p.y);
        let new_idx = map_idx(new_p.x, new_p.y);
        self.map.occupation[old_idx] = None;
        self.map.occupation[new_idx] = Some(entity);
    }
}

pub fn build_map(mut commands: Commands) {
    let mb = MapBuilder::new();
    commands.insert_resource(mb);
}
