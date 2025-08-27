use super::prelude::*;

pub struct State {
    pub map: Map,
    pub player: Player,
}

impl State {
    pub fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &mut self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}
