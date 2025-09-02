// use crate::camera;

use super::prelude::*;

const NUM_TILES: usize = (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_idx(point: &Point) -> usize {
    // 确保坐标在有效范围内，避免负数转换为usize导致溢出
    let x = point.x.max(0).min(DISPLAY_WIDTH - 1) as usize;
    let y = point.y.max(0).min(DISPLAY_HEIGHT - 1) as usize;
    (y * DISPLAY_WIDTH as usize) + x
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES],
        }
    }

    // pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
    //     ctx.set_active_console(0); // 第一个图层显示地图
    //     // 仅接受可视边界
    //     for y in camera.top_y..camera.bottom_y {
    //         for x in camera.left_x..camera.right_x {
    //             if self.is_in_bounds(&Point::new(x, y)) {
    //                 let idx = map_idx(&Point::new(x, y));
    //                 match self.tiles[idx] {
    //                     TileType::Floor => {
    //                         ctx.set(
    //                             x - camera.left_x,
    //                             y - camera.top_y,
    //                             WHITE,
    //                             BLACK,
    //                             to_cp437('.'),
    //                         );
    //                     }
    //                     TileType::Wall => {
    //                         ctx.set(
    //                             x - camera.left_x,
    //                             y - camera.top_y,
    //                             WHITE,
    //                             BLACK,
    //                             to_cp437('#'),
    //                         );
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // 是否在地图范围内
    pub fn is_in_bounds(&self, point: &Point) -> bool {
        point.x > 0 && point.x < DISPLAY_WIDTH && point.y > 0 && point.y < DISPLAY_HEIGHT
    }

    // 是否可以进入该方块(也就是地板)
    pub fn can_enter_tile(&self, point: &Point) -> bool {
        self.is_in_bounds(point) && self.tiles[map_idx(point)] == TileType::Floor
    }

    // 尝试获取方块的索引
    pub fn try_idx(&self, point: &Point) -> Option<usize> {
        if !self.is_in_bounds(point) {
            None
        } else {
            Some(map_idx(point))
        }
    }

    // 判断出口是否有效
    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.is_in_bounds(&destination) {
            if self.can_enter_tile(&destination) {
                let idx = map_idx(&destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0));
        }
        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }

    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] != TileType::Floor
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(DISPLAY_WIDTH, DISPLAY_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.is_in_bounds(&point)
    }
}
