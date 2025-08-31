use crate::prelude::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;

#[system]
#[read_component(Health)]
#[read_component(WantsToAttack)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attacks = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity)> = attacks
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();
    victims.iter().for_each(|(message, victiom)| {
        if let Ok(mut health) = ecs
            .entry_mut(*victiom)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack:{}", health.current);
            health.current -= 1;
            println!("Health after attack:{}", health.current);
            if health.current <= 0 {
                println!("Victim is dead");
                commands.remove(*victiom);
            }
            commands.remove(*message);
        }
    });
}
