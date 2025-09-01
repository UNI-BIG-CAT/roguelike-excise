// mod collision;
mod chasing;
mod combat;
mod end_turn;
mod entity_render;
mod hub;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod tooltips;
use super::prelude::*;
// use crate::systems::collision::collision_system;
use crate::systems::entity_render::entity_render_system;
pub use chasing::*;
pub use combat::*;
pub use end_turn::*;
pub use hub::*;
pub use map_render::*;
pub use movement::*;
pub use player_input::*;
pub use random_move::*;
pub use tooltips::*;

pub fn build_input_scheduler() -> Schedule {
    // 运行所有注册的游戏系统
    // 处理输入、更新游戏状态、准备渲染数据
    Schedule::builder()
        .add_system(player_input_system())
        .flush() //数据更新后需要flush
        .add_system(hud_system())
        .add_system(tooltips_system())
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    // 处理输入、更新游戏状态、准备渲染数据
    Schedule::builder()
        .add_system(combat_system())
        .flush()
        .add_system(movement_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(hud_system())
        .add_system(tooltips_system())
        .add_system(end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move_system())
        .add_system(chasing_system())
        .flush()
        .add_system(combat_system())
        .flush()
        .add_system(movement_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(hud_system())
        .add_system(tooltips_system())
        .add_system(end_turn_system())
        .build()
}
