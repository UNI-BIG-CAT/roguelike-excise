use super::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_idx(point: &Point) -> usize {
    (point.y as usize * SCREEN_WIDTH as usize) + point.x as usize
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

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(&Point { x, y });
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }

    fn is_in_bounds(&self, point: &Point) -> bool {
        point.x > 0 && point.x < SCREEN_WIDTH && point.y > 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: &Point) -> bool {
        self.is_in_bounds(point) && self.tiles[map_idx(point)] == TileType::Floor
    }

    pub fn try_idx(&self, point: &Point) -> Option<usize> {
        if !self.is_in_bounds(point) {
            None
        } else {
            Some(map_idx(point))
        }
    }
}
