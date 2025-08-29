mod collision;
mod end_turn;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;

use super::prelude::*;
use crate::systems::collision::collision_system;
use crate::systems::entity_render::entity_render_system;
pub use end_turn::*;
pub use map_render::*;
pub use player_input::*;
pub use random_move::*;

pub fn build_input_scheduler() -> Schedule {
    // 运行所有注册的游戏系统
    // 处理输入、更新游戏状态、准备渲染数据
    Schedule::builder()
        .add_system(player_input_system())
        .flush() //数据更新后需要flush
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    // 处理输入、更新游戏状态、准备渲染数据
    Schedule::builder()
        .add_system(collision_system())
        .flush()
        .add_system(player_input_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move_system())
        .add_system(collision_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(end_turn_system())
        .build()
}
