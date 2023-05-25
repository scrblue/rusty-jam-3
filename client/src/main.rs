use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use connect_menu::{connect_menu, ConnectMenuState};
use in_game::{
    events::{
        connect_events, disconnect_events, handle_entity_assignment, reject_events,
        spawning::listen_character_creation, tick_events,
    },
    init_game,
    input::key_input,
    physics::{restep_physics, step_physics},
    sync::{sync_camera_pos, sync_physics, sync_predicted_sprites},
    InputHistory, OwnedEntities, QueuedCommand,
};
use naia_bevy_client::{ClientConfig, CommandHistory, Plugin as ClientPlugin, ReceiveEvents};

use shared::{physics::PhysicsWorld, protocol};

mod connect_menu;
mod in_game;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum MainState {
    #[default]
    ConnectMenu,
    InGame,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
struct MainLoop;
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SystemSet)]
struct Tick;

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin::new(ClientConfig::default(), protocol()))
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_state::<MainState>()
        .insert_resource(PhysicsWorld::default())
        // Connect Menu
        .insert_resource(ConnectMenuState::default())
        .add_system(connect_menu.in_set(OnUpdate(MainState::ConnectMenu)))
        // In Game
        .add_startup_system(init)
        .add_system(init_game.in_schedule(OnEnter(MainState::InGame)))
        .add_systems(
            (
                connect_events,
                disconnect_events,
                handle_entity_assignment,
                reject_events,
                listen_character_creation,
                restep_physics,
            )
                .chain()
                .in_set(ReceiveEvents),
        )
        .configure_set(Tick.after(ReceiveEvents))
        .add_system(tick_events.in_set(Tick))
        .configure_set(MainLoop.after(Tick))
        .add_systems(
            (
                key_input,
                // sync_camera_pos,
                sync_predicted_sprites,
                sync_physics,
                // sync_physics_state,
            )
                .chain()
                .in_set(MainLoop),
        )
        .add_system(
            step_physics
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(MainLoop),
        )
        .insert_resource(FixedTime::new(Duration::from_millis(50)))
        .run()
}

pub fn init(mut commands: Commands) {
    commands.insert_resource(OwnedEntities {
        player_avatar: None,
    });
    commands.insert_resource(QueuedCommand { command: None });
    commands.insert_resource(InputHistory {
        history: CommandHistory::default(),
    });

    commands.spawn(Camera2dBundle::default());
}
