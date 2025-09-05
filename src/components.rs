use super::prelude::*;
use std::collections::HashSet;

//
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}
//
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Player {
    pub map_level: u32,
}
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Enemy;
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct MovingRandomly;

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct ChasingPlayer;

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Item;
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct AmuletOfYala;

#[derive(Debug, Clone, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: self.visible_tiles.clone(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct ProvidersHealing {
    pub amount: i32,
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct ProvidesDungeonMap;

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Carried(pub Entity);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}
