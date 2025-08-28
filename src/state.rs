use super::prelude::*;

pub struct State {
    // pub map: Map,
    // pub player: Player,
    // pub camera: Camera,
    ecs: World,
    resources: Resources,
    system: Schedule,
}

impl State {
    pub fn new() -> Self {
        // let mut rng = RandomNumberGenerator::new();
        // let map_builder = MapBuilder::new(&mut rng);
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            // map: map_builder.map,
            // player: Player::new(map_builder.player_start),
            // camera: Camera::new(map_builder.player_start),
            ecs,
            resources,
            system: build_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        //
        // self.player.update(ctx, &mut self.map, &mut self.camera);
        // self.map.render(ctx, &self.camera);
        // self.player.render(ctx, &self.camera);

        self.resources.insert(ctx.key); // 把键盘输入状态作为一个资源添加到资源列表(会替换掉已经存在的同类型资源)
        self.system.execute(&mut self.ecs, &mut self.resources);

        // render Draw Buffer
        self.system.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}
