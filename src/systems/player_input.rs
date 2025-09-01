use crate::prelude::systems::*;
use crate::prelude::*;
use legion::world::SubWorld;

/*
实际工作流程
    游戏启动时：
    Legion 框架扫描所有带有 #[system] 的函数
    自动注册这些系统
游戏运行时：
    框架自动调用 player_input 系统
    系统读取按键输入，更新玩家位置
    系统更新相机位置
无需手动调用：
    你不需要写 player_input() 这样的调用代码
    框架自动处理一切
*/

// #[system] 是一个属性宏（Attribute Macro），它来自 Legion ECS（Entity Component System）框架。
// 告诉 Legion ECS 框架："这是一个系统函数，应该被注册到游戏循环中自动执行"。
#[system]
#[write_component(Point)] // 表示这个系统需要写入 Point 组件 允许系统修改实体的位置
#[read_component(Player)] // 表示这个系统需要读取 Player 组件 允许系统识别哪些实体是玩家
#[read_component(Enemy)] // 表示这个系统需要读取 Enemy 组件 允许系统识别哪些实体是敌人
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld, // 自动注入 ECS 世界
    commands: &mut CommandBuffer,
    // #[resource] 表示这个参数应该从全局资源中获取
    #[resource] key: &Option<VirtualKeyCode>, // 自动注入按键输入
    #[resource] turn_state: &mut TurnState,   // 自动注入游戏回合状态
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();
        if delta.x != 0 || delta.y != 0 {
            let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
            let mut hit_something = false;
            let mut did_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    did_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });
            if !hit_something {
                did_something = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
            if did_something {
                if let Ok(health) = ecs
                    .entry_mut(player_entity)
                    .unwrap()
                    .get_component_mut::<Health>()
                {
                    health.current = i32::min(health.max, health.current + 1);
                }
            }

            *turn_state = TurnState::PlayerTurn;
        }
        // players.iter(ecs).for_each(|(entity, pos)| {
        //     let destination = *pos + delta;
        //     commands.push((
        //         (),
        //         WantsToMove {
        //             entity: *entity,
        //             destination,
        //         },
        //     ));
        //     *turn_state = TurnState::PlayerTurn;
        // });
    }
}
