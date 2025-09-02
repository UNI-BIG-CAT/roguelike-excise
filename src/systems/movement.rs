use crate::prelude::systems::*;
use crate::prelude::*;
use legion::world::SubWorld;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    wants_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    command: &mut CommandBuffer,
) {
    if let Ok(entry) = ecs.entry_ref(wants_move.entity) {
        if let Ok(fov) = entry.get_component::<FieldOfView>() {
            command.add_component(wants_move.entity, fov.clone_dirty());
        }
        if entry.get_component::<Player>().is_ok() {
            camera.on_player_move(wants_move.destination);
        }
    }

    if map.can_enter_tile(&wants_move.destination) {
        command.add_component(wants_move.entity, wants_move.destination);
        if ecs
            .entry_ref(wants_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(wants_move.destination);
        }
    }
    command.remove(*entity);
}
