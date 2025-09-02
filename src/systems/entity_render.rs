use crate::prelude::*;
use legion::world::SubWorld;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn entity_render(ecs: &mut SubWorld, #[resource] camera: &Camera) {
    // 每一个需要往终端窗口写入数据的系统都要开启一个新的DrawBatch
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);
    //
    let mut renderables = <(&Point, &Render)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    renderables
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    // renderables // 查询所有同时包含Point和Render组件的实体
    //     .iter(ecs) // 指定哪一个subWorld要被查询,并且转化为迭代器
    //     .for_each(|(pos, render)| {
    //         draw_batch.set(*pos - offset, render.color, render.glyph);
    //     });
    draw_batch.submit(1).expect("Batch error");
}
