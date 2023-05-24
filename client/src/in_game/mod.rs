//! This is the root modul for the "in-game" state.

use bevy::prelude::*;
use naia_bevy_client::{transport::webrtc, Client, CommandHistory};
use shared::messages::{Auth, PlayerInput};

use crate::connect_menu::ConnectMenuState;

pub mod events;
pub mod input;
pub mod physics;
pub mod sync;

/// Utility type defining a pair of [`Entity`] instances which both represent the same remote
/// [`Entity`] on the server.
///
/// The `confirmed` entity has the last known authoritative state, while the `predicted` entity has
/// a state which is predicted by player input and extrapolation.
pub struct EntityProxy {
    pub confirmed: Entity,
    pub predicted: Entity,
}

/// This resource contains all [`EntityProxy`] instances for entities "owned" by this player.
///
/// While the server remain authoritative, this allows keeping track of what the player should be in
/// control of.
#[derive(Resource)]
pub struct OwnedEntities {
    pub player_avatar: Option<EntityProxy>,
}

/// This resource is the next command to be sent to the server. It is set from player input.
#[derive(Resource)]
pub struct QueuedCommand {
    pub command: Option<PlayerInput>,
}

/// This resource keeps track of a player's recent commands such as to be able to replay them and
/// correct predictions
#[derive(Resource)]
pub struct InputHistory {
    pub history: CommandHistory<PlayerInput>,
}

/// A marker trait for confirmed entities to point to their predicted counterpart
#[derive(Component)]
pub struct Confirmed(Entity);

/// A marker trait for predicted entities.
#[derive(Component)]
pub struct Predicted;

/// A simple initialization system for the in-game state.
pub fn init_game(conn: Res<ConnectMenuState>, mut client: Client, mut commands: Commands) {
    client.auth(Auth {
        name: conn.user.clone(),
        channel_password: conn.pass.clone(),
    });

    let socket = webrtc::Socket::new(&conn.addr, client.socket_config());
    client.connect(socket);
}
