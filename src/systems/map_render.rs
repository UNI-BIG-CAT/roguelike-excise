use crate::prelude::*; // super用于从当前模块向上访问parent模块，所以只能用crate
use legion::world::SubWorld;

#[system]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn map_render(ecs: &mut SubWorld, #[resource] map: &Map, #[resource] camera: &Camera) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.is_in_bounds(&pt) && player_fov.visible_tiles.contains(&pt) {
                let idx = map_idx(&pt);
                let glypg = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                draw_batch.set(pt - offset, ColorPair::new(WHITE, BLACK), glypg);
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
