#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod constants;
mod map;
mod map_builder;
mod player;
mod state;
mod prelude {
    pub use super::constants::*;
    pub use super::map::*;
    pub use super::map_builder::*;
    pub use super::player::*;
    pub use super::state::*;
    pub use bracket_lib::prelude::*;
}

use prelude::*;

fn main() {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .with_fps_cap(30.0) // 自动调控游戏的运行速度，会告知操作系统游戏程序可以在两帧之间暂停运行，防止游戏过快，也可以缓解cpu压力
        .build()
        .unwrap();
    main_loop(context, State::new()).expect("Game over");
}
