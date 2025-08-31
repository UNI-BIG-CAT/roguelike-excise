#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod camera;
mod components;
mod constants;
mod map;
mod map_builder;
mod spawner;
mod state;
mod systems;
mod turn_state;
// mod player;

mod prelude {
    pub use super::camera::*;
    pub use super::components::*;
    pub use super::constants::*;
    pub use super::map::*;
    pub use super::map_builder::*;
    pub use super::spawner::*;
    pub use super::state::*;
    pub use super::systems::*;
    pub use super::turn_state::*;
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    // pub use super::player::*;
}

use prelude::*;

fn main() {
    let context = BTermBuilder::new()
        .with_title("Roguelike Tutorial")
        .with_fps_cap(30.0) // 自动调控游戏的运行速度，会告知操作系统游戏程序可以在两帧之间暂停运行，防止游戏过快，也可以缓解cpu压力
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) //指定后续将要添加的控制台尺寸
        .with_tile_dimensions(32, 32) // 每个字符占32x32像素
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32) // 加载字体文件，每个字符32x32
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // 主控制台 图层0
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // 无背景控制台 图层1
        .with_simple_console_no_bg(SCREEN_HEIGHT * 2, SCREEN_HEIGHT * 2, "terminal8x8.png") // 无背景控制台 图层2
        .with_vsync(true) // 添加垂直同步
        .build()
        .unwrap();
    main_loop(context, State::new()).expect("Game over");
}
