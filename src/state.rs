use super::prelude::*;

pub struct State {
    pub map: Map,
}

impl State {
    pub fn new() -> Self {
        Self { map: Map::new() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }
}
