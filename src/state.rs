use super::prelude::*;
use legion::systems::CommandBuffer;
use std::collections::HashSet;
pub struct State {
    // pub map: Map,
    // pub player: Player,
    // pub camera: Camera,
    ecs: World,
    resources: Resources,
    input_system: Schedule,
    player_system: Schedule,
    monster_system: Schedule,
}

impl State {
    pub fn new() -> Self {
        // let mut rng = RandomNumberGenerator::new();
        // let map_builder = MapBuilder::new(&mut rng);
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start); // 添加玩家

        // let player_entity = *<Entity>::query()
        //     .filter(component::<Player>())
        //     .iter(&mut self.ecs)
        //     .nth(0);

        // spawn_amulet_of_yala(&mut ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        // map_builder.monster_spawns.iter().for_each(|pos| {
        //     spawn_entity(&mut ecs, &mut rng, *pos);
        // });
        spawn_level(&mut ecs, &mut rng, 0, &map_builder.monster_spawns);
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| {
                // spawn_level(&mut ecs, &mut rng, 0, &map_builder.monster_spawns);
                spawn_entity(&mut ecs, &mut rng, pos);
            });

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.theme);
        Self {
            // map: map_builder.map,
            // player: Player::new(map_builder.player_start),
            // camera: Camera::new(map_builder.player_start),
            ecs,
            resources,
            input_system: build_input_scheduler(),
            player_system: build_player_scheduler(),
            monster_system: build_monster_scheduler(),
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs, map_builder.player_start);
        // spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        // map_builder.monster_spawns.iter().for_each(|pos| {
        //     spawn_entity(&mut self.ecs, &mut rng, *pos);
        // });
        spawn_level(&mut self.ecs, &mut rng, 0, &map_builder.monster_spawns);

        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| {
                spawn_level(&mut self.ecs, &mut rng, 0, &map_builder.monster_spawns);
                // spawn_entity(&mut self.ecs, &mut rng, pos);
            });
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "You Died!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to an end.",
        );
        ctx.print_color_centered(5, YELLOW, BLACK, "Don't worry, you can try again.");
        ctx.print_color_centered(8, GREEN, BLACK, "Press to play again");
        if let Some(_key) = ctx.key {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You Win!");
        ctx.print_color_centered(4, WHITE, BLACK, "You've slain the great evil");
        ctx.print_color_centered(
            6,
            YELLOW,
            BLACK,
            "You put the amulet of yala in the right place",
        );
        ctx.print_color_centered(8, GREEN, BLACK, "Press to play again");
        if let Some(_key) = ctx.key {
            self.reset_game_state();
        }
    }

    fn advance_level(&mut self) {
        // 复制玩家和物品
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&mut self.ecs)
            .nth(0)
            .unwrap();
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);

        <(Entity, &Carried)>::query()
            .iter(&mut self.ecs)
            .filter(|(_e, carry)| carry.0 == player_entity)
            .map(|(e, _carry)| *e)
            .for_each(|entity| {
                entities_to_keep.insert(entity);
            });
        // 删除其余实体
        let mut cb = CommandBuffer::new(&mut self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(&e) {
                cb.remove(*e);
            }
        }
        cb.flush(&mut self.ecs);
        // 将视场设置为脏
        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| {
                fov.is_dirty = true;
            });
        //创建新地图
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            });
        if map_level == 2 {
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }
        // map_builder.monster_spawns.iter().for_each(|pos| {
        //     spawn_enemy(&mut self.ecs, &mut rng, *pos);
        // });
        spawn_level(
            &mut self.ecs,
            &mut rng,
            map_level as usize,
            &map_builder.monster_spawns,
        );

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        //
        // self.player.update(ctx, &mut self.map, &mut self.camera);
        // self.map.render(ctx, &self.camera);
        // self.player.render(ctx, &self.camera);

        self.resources.insert(ctx.key); // 把键盘输入状态作为一个资源添加到资源列表(会替换掉已经存在的同类型资源)
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
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
        // self.system.execute(&mut self.ecs, &mut self.resources);
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_system
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_system
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_system
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => {
                self.advance_level();
            }
        }
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
