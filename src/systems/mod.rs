mod entity_render;
mod map_render;
mod play_input;

use crate::systems::entity_render::entity_render_system;

use super::prelude::*;
pub use map_render::*;
pub use play_input::*;

pub fn build_scheduler() -> Schedule {
    // 运行所有注册的游戏系统
    // 处理输入、更新游戏状态、准备渲染数据
    Schedule::builder()
        .add_system(player_input_system())
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .build()
}
