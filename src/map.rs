use crate::camera;

use super::prelude::*;

const NUM_TILES: usize = (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_idx(point: &Point) -> usize {
    (point.y as usize * DISPLAY_WIDTH as usize) + point.x as usize
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0); // 第一个图层显示地图
        // 仅接受可视边界
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.is_in_bounds(&Point::new(x, y)) {
                    let idx = map_idx(&Point::new(x, y));
                    match self.tiles[idx] {
                        TileType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('.'),
                            );
                        }
                        TileType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('#'),
                            );
                        }
                    }
                }
            }
        }
    }

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
}
