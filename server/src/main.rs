use std::{net::SocketAddr, time::Duration};

use bevy::{
    app::ScheduleRunnerSettings, diagnostic::DiagnosticsPlugin, log::LogPlugin, prelude::*,
    scene::ScenePlugin, time::TimePlugin,
};
use bevy_rapier2d::prelude::*;
use clap::Parser;
use naia_bevy_server::{
    transport::webrtc, Plugin as ServerPlugin, ReceiveEvents, Server, ServerConfig,
};

use shared::protocol;

use resources::{MainRoomKey, UserAvatarMapping, UserNameMapping};
use server_event_handling::{
    auth_events, connect_events, disconnect_events, error_events, sync_physics, tick_events,
};

mod resources;
mod server_event_handling;

#[derive(Parser, Resource)]
pub struct Args {
    #[arg(short, long)]
    addr: SocketAddr,
    #[arg(short, long)]
    webrtc_addr: SocketAddr,

    #[arg(short, long)]
    max_players: u8,
    #[arg(short, long)]
    password: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut cfg = RapierConfiguration::default();
    cfg.gravity = Vec2::new(0.0, 0.0);

    App::default()
        .add_plugins(MinimalPlugins)
        .add_plugin(AssetPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(TransformPlugin)
        .add_plugin(HierarchyPlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(ScenePlugin)
        .add_plugin(ServerPlugin::new(ServerConfig::default(), protocol()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(cfg)
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(3)))
        .insert_resource(args)
        .add_startup_system(init)
        .add_systems(
            (
                auth_events,
                connect_events,
                disconnect_events,
                error_events,
                tick_events,
            )
                .chain()
                .in_set(ReceiveEvents),
        )
        .add_system(sync_physics)
        .run();
}

fn init(cfg: Res<Args>, mut server: Server, mut commands: Commands) {
    info!("Initializing server");
    let addrs = webrtc::ServerAddrs::new(
        cfg.addr,
        cfg.webrtc_addr,
        &format!("http://{}", cfg.webrtc_addr),
    );
    let socket = webrtc::Socket::new(&addrs, server.socket_config());

    server.listen(socket);

    let main_room_key = server.make_room().key();

    commands.insert_resource(MainRoomKey(main_room_key));
    commands.insert_resource(UserAvatarMapping::new());
    commands.insert_resource(UserNameMapping::new());
}
