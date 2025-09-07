mod automata;
mod drunkard;
mod empty;
mod rooms;
mod themes;

use super::prelude::*;
use crate::spawner;
use automata::*;
use drunkard::*;
use empty::*;
use rooms::*;
use std::boxed::Box;
use themes::*;

trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub trait MapTheme: Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType;
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
    pub theme: Box<dyn MapTheme>,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        // let mut architect = CellularAutomataArchitect {};
        // let mut architect = DrunkardArchitect {};
        let mut architect = RoomsArchitect {};
        // let mut architect: Box<dyn MapArchitect> = match rng.range(0, 2) {
        //     0 => Box::new(CellularAutomataArchitect {}),
        //     1 => Box::new(RoomsArchitect {}),
        //     // 2 => Box::new(DrunkardArchitect {}),
        //     _ => unreachable!(),
        // };
        let mut mb = architect.new(rng);
        // apply_prefab(&mut architect, rng);
        mb.theme = match rng.range(0, 2) {
            0 => DungeonTheme::new(),
            1 => ForestTheme::new(),
            _ => unreachable!(),
        };
        mb
    }

    pub fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    pub fn find_most_distant(&self) -> Point {
        // 放战利品
        let dijkstra_map = DijkstraMap::new(
            DISPLAY_WIDTH,
            DISPLAY_HEIGHT,
            &vec![self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
        );

        const UNREACHABLE: &f32 = &f32::MAX;

        // 查找最远的可达点
        if let Some((index, _)) = dijkstra_map
            .map
            .iter()
            .enumerate()
            .filter(|(_, dist)| *dist < UNREACHABLE)
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        {
            self.map.index_to_point2d(index)
        } else {
            // 如果没有可达点，返回地图右下角作为默认位置
            Point::new(DISPLAY_WIDTH - 2, DISPLAY_HEIGHT - 2)
        }
    }

    pub fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS as usize {
            let room = Rect::with_size(
                rng.range(1, DISPLAY_WIDTH - 10),
                rng.range(1, DISPLAY_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < DISPLAY_WIDTH && p.y > 0 && p.y < DISPLAY_HEIGHT {
                        let idx = map_idx(&p);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    pub fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(&Point { x, y }) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    pub fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(&Point { x, y }) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    pub fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
            }
        }
    }

    fn spawn_monsters(&mut self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_MONSTERS: usize = 30;
        let mut spawnable_tites: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == TileType::Floor
                    && DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();
        let mut spawns = Vec::new();
        for _ in 0..NUM_MONSTERS {
            let target_index = rng.random_slice_index(&spawnable_tites).unwrap();
            spawns.push(spawnable_tites[target_index].clone());
            spawnable_tites.remove(target_index);
        }

        spawns
    }
}
