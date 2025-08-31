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
