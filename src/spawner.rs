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
        Health {
            current: 20,
            max: 20,
        },
    ));
}

pub fn spawn_enemy(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (_hp, name, _glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };
    let glyph = match rng.range(0, 4) {
        0 => to_cp437('E'), // 双头怪
        1 => to_cp437('O'), // 食人魔
        2 => to_cp437('o'), // 兽人
        3 => to_cp437('g'), // 妖精
        _ => unreachable!(),
    };
    if glyph == to_cp437('g') {
        // 妖精使用随机移动
        ecs.push((
            Enemy,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph,
            },
            // ChasingPlayer {},
            MovingRandomly {},
            Health {
                current: 10,
                max: 10,
            },
            Name(name),
        ));
    } else {
        // 其他敌人追踪玩家
        ecs.push((
            Enemy,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph,
            },
            ChasingPlayer {},
            Health {
                current: 10,
                max: 10,
            },
            Name(name),
        ));
    }
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}
