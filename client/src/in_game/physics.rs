use std::collections::BTreeMap;

use bevy::{prelude::*, utils::HashMap};
use naia_bevy_client::{
    events::UpdateComponentEvents, sequence_greater_than, Client, Replicate, Tick,
};
use rapier2d::prelude::{nalgebra, vector};
use shared::{
    components::PhysicsStateSync,
    physics::{components::PhysicsBodyHandle, PhysicsWorld},
};

use super::{Confirmed, InputHistory, OwnedEntities};

/// Meant to be run with other [`EventReader`]s for naia.
///
/// This listens for updates to [`PhysicsStateSync`] components and reflects changes to to predicted
/// wolrd.
pub fn restep_physics(
    mut reader: EventReader<UpdateComponentEvents>,
    physics_state_query: Query<(&PhysicsStateSync, &Confirmed)>,
    physics_handle_query: Query<&PhysicsBodyHandle>,
    owned_entities: Res<OwnedEntities>,
    mut player_commands: ResMut<InputHistory>,
    mut physics: ResMut<PhysicsWorld>,
    client: Client,
) {
    let mut last_tick = None;
    for events in reader.iter() {
        for (tick, updated_entity) in events.read::<PhysicsStateSync>() {
            let Ok((to_sync, confirmed)) = physics_state_query
                .get(updated_entity) else { continue; };

            let Ok(handles) = physics_handle_query.get(confirmed.0) else { continue; };
            let Some(rb) = physics.get_rigid_body_mut(handles.rigid_body) else { continue; };

            rb.set_translation(vector![*to_sync.pos_x_m, *to_sync.pos_y_m], true);
            rb.set_linvel(vector![*to_sync.linvel_x_m, *to_sync.linvel_y_m], true);

            if let Some(owned) = owned_entities.player_avatar.as_ref() {
                if updated_entity == owned.confirmed {
                    last_tick = if let Some(last_tick) = last_tick {
                        if sequence_greater_than(tick, last_tick) {
                            Some(tick)
                        } else {
                            Some(last_tick)
                        }
                    } else {
                        Some(tick)
                    }
                }
            }
        }
    }

    if let Some(owned) = owned_entities.player_avatar.as_ref() {
        if let Some(mut last_tick) = last_tick {
            last_tick = last_tick.wrapping_sub(1);

            let Ok(handles) = physics_handle_query.get(owned.predicted) else { return; };

            for (cmd_tick, cmd) in player_commands.history.replays(&last_tick) {
                while sequence_greater_than(cmd_tick, last_tick) {
                    physics.step();
                    last_tick += 1;
                }

                let Some(rb) = physics.get_rigid_body_mut(handles.rigid_body) else { return; };
                rb.set_linvel(vector![cmd.x_axis, cmd.y_axis], true);
            }

            while sequence_greater_than(client.client_tick().unwrap(), last_tick) {
                physics.step();
                last_tick += 1;
            }
        }
    }
}

pub fn step_physics(mut physics: ResMut<PhysicsWorld>) {
    physics.step();
}
