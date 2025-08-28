use super::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        // 通过push创建组件
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}
