use crate::prelude::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;

#[system]
#[write_component(Health)]
#[read_component(WantsToAttack)]
#[read_component(Player)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attacks = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity)> = attacks
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();

    victims.iter().for_each(|(message, victim)| {
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();
        //
        println!("Processing victim: {:?}", *victim);
        if let Ok(health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack:{}", health.current);
            health.current -= 1;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            } else {
                println!("Health after attack:{}", health.current);
            }
        } else {
            println!("Failed to get Health component for victim: {:?}", victim);
        }
        commands.remove(*message);
    });
}
