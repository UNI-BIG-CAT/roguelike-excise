use crate::prelude::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;

#[system]
#[write_component(Health)]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[read_component(Damage)]
#[read_component(Carried)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attacks = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity, Entity)> = attacks
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect();

    victims.iter().for_each(|(message, attacker, victim)| {
        let base_damage = if let Ok(v) = ecs.entry_ref(*attacker) {
            if let Ok(dmg) = v.get_component::<Damage>() {
                dmg.0
            } else {
                0
            }
        } else {
            0
        };
        let weapon_damage: i32 = <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, dmg)| dmg.0)
            .sum();
        // println!("weapon_damage: {}", weapon_damage);
        let final_damage = base_damage + weapon_damage;
        // println!("final_damage: {}", final_damage);
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= final_damage;
            if health.current < 1 {
                commands.remove(*victim);
            }
        }
        // Always remove the attack message after processing
        commands.remove(*message);
    });

    // let victims: Vec<(Entity, Entity)> = attacks
    //     .iter(ecs)
    //     .map(|(entity, attack)| (*entity, attack.victim))
    //     .collect();
    // victims.iter().for_each(|(message, victim)| {
    //     let is_player = ecs
    //         .entry_ref(*victim)
    //         .unwrap()
    //         .get_component::<Player>()
    //         .is_ok();
    //     //
    //     if let Ok(health) = ecs
    //         .entry_mut(*victim)
    //         .unwrap()
    //         .get_component_mut::<Health>()
    //     {
    //         health.current -= 1;
    //         if health.current < 1 && !is_player {
    //             commands.remove(*victim);
    //         } else {
    //         }
    //     } else {
    //         println!("Failed to get Health component for victim: {:?}", victim);
    //     }
    //     commands.remove(*message);
    // });
}
