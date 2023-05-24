use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use naia_bevy_server::{
    events::{AuthEvents, ConnectEvent, DisconnectEvent, ErrorEvent, TickEvent},
    CommandsExt, Random, Server,
};

use shared::{
    channels::{GameMessageChannel, PlayerInputChannel},
    components::{CharacterEntity, PhysicsStateSync},
    messages::{Auth, EntityAssignment, PlayerInput},
};

use crate::{
    resources::{MainRoomKey, UserAvatarMapping, UserNameMapping},
    Args,
};

pub fn auth_events(
    mut event_reader: EventReader<AuthEvents>,
    cfg: Res<Args>,
    mut users_names: ResMut<UserNameMapping>,
    mut server: Server,
) {
    for events in event_reader.iter() {
        for (user_key, auth) in events.read::<Auth>() {
            if server.users_count() > cfg.max_players.into() {
                server.reject_connection(&user_key);
            }

            if let Some(password) = &cfg.password {
                if &auth.channel_password != password {
                    server.reject_connection(&user_key);
                }
            }

            if users_names.get_by_name(&auth.name).is_some() {
                server.reject_connection(&user_key);
            }

            users_names.insert(user_key, auth.name);
            server.accept_connection(&user_key);
        }
    }
}

pub fn connect_events(
    mut event_reader: EventReader<ConnectEvent>,
    main_room_key: Res<MainRoomKey>,
    users_names: Res<UserNameMapping>,
    mut users_avatars: ResMut<UserAvatarMapping>,
    mut server: Server,
    mut commands: Commands,
) {
    for ConnectEvent(user_key) in event_reader.iter() {
        let address = server
            .user_mut(user_key)
            .enter_room(&main_room_key.0)
            .address();

        let Some(name) = users_names.get_by_user(&user_key) else { return; };
        info!("User {name} connected on {address}");

        // TODO: Round-robin spawn points?
        let x = Random::gen_range_f32(-2.0, 2.0);
        let y = Random::gen_range_f32(-2.0, 2.0);

        let state = PhysicsStateSync::new_complete(0.0, 0.0, 0.0, x, y, 0.0);

        let entity = commands
            .spawn_empty()
            .enable_replication(&mut server)
            .insert(CharacterEntity)
            .insert(state)
            .insert(Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            })
            .insert(RigidBody::Dynamic)
            .insert(Damping {
                linear_damping: 1.0,
                angular_damping: 1.0,
            })
            .insert(Collider::cuboid(0.5, 0.5))
            .insert(Restitution::coefficient(0.2))
            .insert(TransformBundle::from_transform(Transform::from_xyz(
                x, y, 1.0,
            )))
            .id();

        server.room_mut(&main_room_key.0).add_entity(&entity);

        users_avatars.insert(*user_key, entity);

        let mut assignment_msg = EntityAssignment::new(true);
        assignment_msg.entity.set(&server, &entity);

        server.send_message::<GameMessageChannel, EntityAssignment>(user_key, &assignment_msg);
    }
}

pub fn disconnect_events(
    mut event_reader: EventReader<DisconnectEvent>,
    main_room_key: Res<MainRoomKey>,
    mut users_names: ResMut<UserNameMapping>,
    mut users_avatars: ResMut<UserAvatarMapping>,
    mut server: Server,
    mut commands: Commands,
) {
    for DisconnectEvent(user_key, _user) in event_reader.iter() {
        let Some(name) = users_names.get_by_user(user_key) else { return; };
        info!("User {name} disconnecting");

        if let Some(entity) = users_avatars.get_by_user(user_key) {
            commands.entity(*entity).despawn();
            server.room_mut(&main_room_key.0).remove_entity(entity);
        }

        users_avatars.remove_by_user(user_key);
        users_names.remove_by_user(user_key);
    }
}

pub fn error_events(mut event_reader: EventReader<ErrorEvent>) {
    for ErrorEvent(error) in event_reader.iter() {
        error!("server error: {error}");
    }
}

/// "Main loopt" happens here
pub fn tick_events(
    mut event_reader: EventReader<TickEvent>,
    mut velocity_query: Query<&mut Velocity>,
    transform_query: Query<&Transform>,
    mut position_query: Query<&mut PhysicsStateSync>,
    mut server: Server,
) {
    let mut has_ticked = false;

    for TickEvent(server_tick) in event_reader.iter() {
        has_ticked = true;

        let mut messages = server.receive_tick_buffer_messages(server_tick);
        for (_user_key, input) in messages.read::<PlayerInputChannel, PlayerInput>() {
            let Some(entity) = &input.entity.get(&server) else { continue; };

            let Ok(mut velocity) = velocity_query.get_mut(*entity) else { continue; };
            let Ok(transform) = transform_query.get(*entity) else { continue; };
            let Ok(mut physics) = position_query.get_mut(*entity) else { continue; };

            warn!("{}, {}", input.x_axis, input.y_axis);

            // Do not process invalid inputs
            if input.x_axis > 1.0
                || input.y_axis > 1.0
                || input.x_axis < -1.0
                || input.y_axis < -1.0
            {
                continue;
            }

            // Set velocity
            velocity.linvel.x = input.x_axis;
            velocity.linvel.y = input.y_axis;
            *physics.linvel_x_m = input.x_axis;
            *physics.linvel_y_m = input.y_axis;
            // Update network position
            *physics.pos_x_m = transform.translation.x;
            *physics.pos_y_m = transform.translation.y;
            error!("{}, {}", transform.translation.x, transform.translation.y);
        }
    }

    if has_ticked {
        for (_, user_key, entity) in server.scope_checks() {
            // TODO: Interest management here
            server.user_scope(&user_key).include(&entity);
        }
    }
}

pub fn sync_physics(
    rapier_query: Query<(&Transform, &Velocity)>,
    mut state_query: Query<(Entity, &mut PhysicsStateSync)>,
) {
    for (entity, mut state) in state_query.iter_mut() {
        let Ok((transform, velocity)) = rapier_query.get(entity) else { continue; };

        *state.linvel_x_m = velocity.linvel.x;
        *state.linvel_y_m = velocity.linvel.y;
        *state.pos_x_m = transform.translation.x;
        *state.pos_y_m = transform.translation.y;
    }
}
