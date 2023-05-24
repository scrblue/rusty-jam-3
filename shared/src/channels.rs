use naia_bevy_shared::{
    Channel, ChannelDirection, ChannelMode, ProtocolPlugin, ReliableSettings, TickBufferSettings,
};

/// For client-to-server packets containing a given player's inputs for a tick.
#[derive(Channel)]
pub struct PlayerInputChannel;

/// For "messages" to individual players related to the game. This includes:
///   * Entity assignment
///   * Target assignment
#[derive(Channel)]
pub struct GameMessageChannel;

pub struct ChannelsPlugin;
impl ProtocolPlugin for ChannelsPlugin {
    fn build(&self, protocol: &mut naia_bevy_shared::Protocol) {
        protocol
            .add_channel::<PlayerInputChannel>(
                ChannelDirection::ClientToServer,
                ChannelMode::TickBuffered(TickBufferSettings::default()),
            )
            .add_channel::<GameMessageChannel>(
                ChannelDirection::ServerToClient,
                ChannelMode::UnorderedReliable(ReliableSettings::default()),
            );
    }
}
