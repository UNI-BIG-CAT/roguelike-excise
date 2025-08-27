use super::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &mut Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point { x: -1, y: 0 },
                VirtualKeyCode::Right => Point { x: 1, y: 0 },
                VirtualKeyCode::Up => Point { x: 0, y: -1 },
                VirtualKeyCode::Down => Point { x: 0, y: 1 },
                _ => Point { x: 0, y: 0 },
            };
            let new_position = self.position + delta;
            if map.can_enter_tile(&new_position) {
                self.position = new_position;
            }
        }
    }
}
