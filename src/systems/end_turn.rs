use crate::prelude::*;
use legion::world::SubWorld;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    let amulet_pos = amulet.iter(ecs).nth(0).unwrap();
    //
    let current_state = turn_state.clone();
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };
    //
    player_hp.iter(ecs).for_each(|(hp, pos)| {
        // 游戏结束
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
        // 游戏胜利
        if pos == amulet_pos {
            new_state = TurnState::Victory;
        }
    });

    //
    *turn_state = new_state;
}
