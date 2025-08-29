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
        spawn_player(&mut ecs, map_builder.player_start); // 添加玩家
        map_builder.rooms.iter().skip(1).map(|r| r.center()).for_each(|pos| {
            spawn_enemy(&mut ecs, &mut rng, pos);
        });
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
        /*
            self.system.execute(&mut self.ecs, &mut self.resources);
            作用：执行游戏系统
                运行所有注册的游戏系统（如 player_input、map_render、entity_render 等）
                处理输入、更新游戏状态、准备渲染数据
            具体执行内容：
                player_input_system：处理键盘输入，移动玩家
                map_render_system：准备地图的渲染数据
                entity_render_system：准备实体（如玩家）的渲染数据
            重要特点：
                这些系统不会直接绘制到屏幕
                而是将绘制命令添加到 DrawBatch 中
                相当于"准备画什么，但还没画"
        */
        self.system.execute(&mut self.ecs, &mut self.resources);
        /*
            render_draw_buffer(ctx).expect("Render error");
            作用：将准备好的内容绘制到屏幕
                执行所有在 DrawBatch 中累积的绘制命令
                真正将内容显示在游戏窗口上
            工作流程：
                从 DrawBatch 中取出所有绘制命令
                调用 ctx.set() 等函数实际绘制像素
                清空 DrawBatch，为下一帧做准备
        */
        render_draw_buffer(ctx).expect("Render error");
        /*
            为什么需要两步？
                1. 性能优化
                系统执行和实际绘制分离
                可以批量处理绘制命令，提高效率
                2. 架构清晰
                系统负责逻辑和准备数据
                渲染器负责实际显示
                3. 避免重复绘制
                系统只执行一次，避免重复计算
                渲染只执行一次，避免闪烁
            类比理解
                想象成餐厅点餐：
                第一行：服务员记录所有客人的点餐（系统执行）
                第二行：厨师根据点餐单做菜并端上桌（渲染到屏幕）
        */
    }
}
