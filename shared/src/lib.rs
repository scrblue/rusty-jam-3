use std::time::Duration;

use naia_bevy_shared::{LinkConditionerConfig, Protocol};

pub mod channels;
pub mod components;
pub mod messages;
pub mod physics;

use channels::ChannelsPlugin;
use components::ComponentsPlugin;
use messages::MessagesPlugin;

pub fn protocol() -> Protocol {
    let mut protocol = Protocol::builder();

    protocol
        .tick_interval(Duration::from_millis(50))
        .add_plugin(ChannelsPlugin)
        .add_plugin(MessagesPlugin)
        .add_plugin(ComponentsPlugin);

    #[cfg(debug_assertions)]
    protocol.link_condition(LinkConditionerConfig::average_condition());

    protocol.build()
}
