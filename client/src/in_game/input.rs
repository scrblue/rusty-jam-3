use bevy::prelude::*;
use naia_bevy_client::Client;
use shared::messages::PlayerInput;

use super::{OwnedEntities, QueuedCommand};

pub fn key_input(
    input: Res<Input<KeyCode>>,
    owned_entities: Res<OwnedEntities>,
    mut queued_command: ResMut<QueuedCommand>,
    client: Client,
) {
    let w = input.pressed(KeyCode::W);
    let a = input.pressed(KeyCode::A);
    let s = input.pressed(KeyCode::S);
    let d = input.pressed(KeyCode::D);

    if let Some(owned_entity) = &owned_entities.player_avatar {
        if w || a || s || d {
            *queued_command = QueuedCommand {
                command: Some(PlayerInput::from_wasd(w, a, s, d)),
            };
            queued_command
                .command
                .as_mut()
                .unwrap()
                .entity
                .set(&client, &owned_entity.confirmed);
        }
    }
}
