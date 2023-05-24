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
    mut mut_physics_state_query: Query<&mut PhysicsStateSync, Without<Confirmed>>,
    physics_handle_query: Query<&PhysicsBodyHandle>,
    owned_entities: Res<OwnedEntities>,
    mut player_commands: ResMut<InputHistory>,
    mut physics: ResMut<PhysicsWorld>,
    client: Client,
) {
    let Some(current_tick) = client.client_tick() else { return; };
    let mut last_tick = None;

    for update in reader.iter() {
        for (server_tick, entity) in update.read::<PhysicsStateSync>() {
            let Ok((state, confirmed)) = physics_state_query.get(entity) else { continue; };
            let Ok(mut predicted) = mut_physics_state_query.get_mut(confirmed.0) else { continue; };

            predicted.mirror(&*state);

            let Ok(handle) = physics_handle_query.get(confirmed.0) else { continue; };
            let Some(rb) = physics.get_rigid_body_mut(handle.rigid_body) else { continue; };

            rb.set_linvel(vector![*state.linvel_x_m, *state.linvel_y_m], true);
            rb.set_translation(vector![*state.pos_x_m, *state.pos_y_m], true);

            last_tick = Some(
                last_tick
                    .map(|curr| {
                        if sequence_greater_than(server_tick, curr) {
                            server_tick
                        } else {
                            curr
                        }
                    })
                    .unwrap_or(server_tick),
            );
        }
    }

    if let Some(last_tick) = last_tick {
        for _ in 0..(current_tick - last_tick) {
            physics.step_back();
        }

        let server_tick = last_tick.wrapping_sub(1);
        if let Some(avatar) = &owned_entities.player_avatar {
            let mut last_tick = server_tick;
            for (current_tick, input) in player_commands.history.replays(&server_tick) {
                while current_tick > last_tick {
                    physics.step();
                    last_tick += 1;
                }

                let Ok(handle) = physics_handle_query.get(avatar.predicted) else { continue; };
                let Some(rb) = physics.get_rigid_body_mut(handle.rigid_body) else { continue; };

                rb.set_linvel(vector![input.x_axis, input.y_axis], true);

                let Ok(mut predicted) = mut_physics_state_query
                    .get_mut(avatar.predicted) else { continue; };
                *predicted.linvel_x_m = input.x_axis;
                *predicted.linvel_y_m = input.y_axis;

                physics.step();
            }
        }
    }
}

pub fn step_physics(mut physics: ResMut<PhysicsWorld>) {
    physics.step();
}
