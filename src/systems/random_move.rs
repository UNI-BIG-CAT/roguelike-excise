use crate::prelude::systems::*;
use crate::prelude::*;
use legion::world::SubWorld;
#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query().filter(component::<Enemy>());
    let mut positions = <(Entity, &Point, &Health)>::query();
    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let delta = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            3 => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        let destination = *pos + delta;
        commands.push((
            (),
            WantsToMove {
                entity: *entity,
                destination,
            },
        ));
        //
        let mut attacked = false;
        positions
            .iter(ecs)
            .filter(|(_, target_pos, _)| **target_pos == destination)
            .for_each(|(victim, _, _)| {
                if ecs
                    .entry_ref(*victim)
                    .unwrap()
                    .get_component::<Player>()
                    .is_ok()
                {
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: *entity,
                            victim: *victim,
                        },
                    ));
                    attacked = true;
                }
            });
        if !attacked {
            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        }
    });
}
