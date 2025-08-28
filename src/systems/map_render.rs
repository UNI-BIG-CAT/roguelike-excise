use crate::prelude::*; // super用于从当前模块向上访问parent模块，所以只能用crate

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for x in camera.top_y..=camera.bottom_y {
        for y in camera.left_x..=camera.right_x {
            let pt = Point::new(x, y);
            let idx = map_idx(&pt);
            let glypg = match map.tiles[idx] {
                TileType::Floor => to_cp437('.'),
                TileType::Wall => to_cp437('#'),
            };
            draw_batch.set(pt, ColorPair::new(WHITE, BLACK), glypg);
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
