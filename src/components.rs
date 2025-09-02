use super::prelude::*;

//
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}
//
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Player;
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
