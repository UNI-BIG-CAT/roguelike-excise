use crate::camera;

use super::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(1); // 第二个图层显示玩家

        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &mut Map, camera: &mut Camera) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point { x: -1, y: 0 },
                VirtualKeyCode::Right => Point { x: 1, y: 0 },
                VirtualKeyCode::Up => Point { x: 0, y: -1 },
                VirtualKeyCode::Down => Point { x: 0, y: 1 },
                _ => Point { x: 0, y: 0 },
            };
            let new_position = self.position + delta;
            // 如果可以进入该方块,把新位置赋值给玩家
            if map.can_enter_tile(&new_position) {
                self.position = new_position;
                camera.on_player_move(new_position);
            }
        }
    }
}
