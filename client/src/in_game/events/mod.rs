//! The module for handling `naia` events emit by the remote server.

use bevy::prelude::*;
use naia_bevy_client::{
    events::{
        ClientTickEvent, ConnectEvent, DisconnectEvent, MessageEvents, RejectEvent,
        UpdateComponentEvents,
    },
    Client,
};
use rapier2d::prelude::{nalgebra, vector};
use shared::{
    channels::{GameMessageChannel, PlayerInputChannel},
    components::PhysicsStateSync,
    messages::{EntityAssignment, PlayerInput},
    physics::{components::PhysicsBodyHandle, PhysicsWorld},
};

use super::{Confirmed, EntityProxy, InputHistory, OwnedEntities, Predicted, QueuedCommand};

pub mod spawning;

/// Fired on sucessfully connecting to the server
pub fn connect_events(mut event_reader: EventReader<ConnectEvent>, mut client: Client) {
    for _ in event_reader.iter() {
        let Ok(server_address) = client.server_address() else { return };
        info!("Connected to {server_address}");
    }
}

/// Fired on being rejected entry to the server
pub fn reject_events(mut event_reader: EventReader<RejectEvent>) {
    for _ in event_reader.iter() {
        warn!("Connection rejected by server");
    }
}

/// Fired on disconnection.
pub fn disconnect_events(mut event_reader: EventReader<DisconnectEvent>) {
    for _ in event_reader.iter() {
        info!("Disconnected from server");
    }
}

/// Fired each tick. This system will:
///   * Remove queued commands from the buffer and transmit them after inserting them into the
///     command history
///   * Update the [`PhysicsStateSync`] for the prediced entity
pub fn tick_events(
    mut event_reader: EventReader<ClientTickEvent>,
    query: Query<&PhysicsBodyHandle, With<Predicted>>,
    owned_entities: Res<OwnedEntities>,
    mut queued_command: ResMut<QueuedCommand>,
    mut input_history: ResMut<InputHistory>,
    mut physics: ResMut<PhysicsWorld>,
    mut client: Client,
) {
    let Some(command) = queued_command.command.take() else { return; };
    let Some(avatar) = &owned_entities.player_avatar else { return; };

    for ClientTickEvent(tick) in event_reader.iter() {
        if !input_history.history.can_insert(tick) {
            continue;
        }

        input_history.history.insert(*tick, command.clone());
        client.send_tick_buffer_message::<PlayerInputChannel, PlayerInput>(tick, &command);
    }
}

/// Fired every time an [`EntityAssignment`] message is sent
pub fn handle_entity_assignment(
    mut event_reader: EventReader<MessageEvents>,
    confirmed_query: Query<&Confirmed>,
    mut owned_entities: ResMut<OwnedEntities>,
    client: Client,
) {
    for events in event_reader.iter() {
        for message in events.read::<GameMessageChannel, EntityAssignment>() {
            let Some(confirmed) = message.entity.get(&client) else { continue; };
            let Ok(predicted) = confirmed_query.get(confirmed).map(|p| p.0) else { continue; };

            if message.assign {
                owned_entities.player_avatar = Some(EntityProxy {
                    confirmed,
                    predicted,
                });
            } else {
                let mut disowned = false;

                if let Some(owned) = &owned_entities.player_avatar {
                    if owned.confirmed == confirmed {
                        disowned = true;
                    }
                }

                if disowned {
                    owned_entities.player_avatar = None;
                }
            }
        }
    }
}
