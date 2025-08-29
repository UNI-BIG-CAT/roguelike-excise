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
pub fn player_input(
    ecs: &mut SubWorld, // 自动注入 ECS 世界
    // #[resource] 表示这个参数应该从全局资源中获取
    #[resource] map: &Map,                    // 自动注入资源
    #[resource] key: &Option<VirtualKeyCode>, // 自动注入按键输入
    #[resource] camera: &mut Camera,          // 自动注入相机
    #[resource] turn_state: &mut TurnState,   // 自动注入游戏回合状态
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        if delta.x != 0 || delta.y != 0 {
            let mut players = <&mut Point>::query().filter(component::<Player>());
            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(&destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                    *turn_state = TurnState::PlayerTurn;
                }
            });
        }
    }
}
