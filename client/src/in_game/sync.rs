//! Synchronization related systems mostly. Getting the sprites to follow the predictions.

use bevy::prelude::*;
use naia_bevy_client::Client;
use rapier2d::prelude::{nalgebra, vector};
use shared::{
    components::PhysicsStateSync,
    physics::{components::PhysicsBodyHandle, PhysicsWorld},
};

use super::{OwnedEntities, Predicted};

/// Copied almost verbatim from the `naia` Bevy demo.
#[derive(Component)]
pub struct Lerp {
    interpolation: f32,
    pub interp_x: f32,
    pub interp_y: f32,

    last_x: f32,
    last_y: f32,
    pub next_x: f32,
    pub next_y: f32,
}

impl Lerp {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            interpolation: 0.0,
            interp_x: x,
            interp_y: y,
            last_x: x,
            last_y: y,
            next_x: x,
            next_y: y,
        }
    }

    pub fn next_pos(&mut self, x: f32, y: f32) {
        self.interpolation = 0.0;
        self.last_x = self.next_x;
        self.last_y = self.next_y;
        self.interp_x = self.next_x;
        self.interp_y = self.next_y;

        self.next_x = x;
        self.next_y = y;
    }

    pub fn interpolate(&mut self, interpolation: f32) {
        if self.interpolation > 1.0 || interpolation == 0.0 {
            return;
        }

        if self.interpolation < interpolation {
            self.interpolation = interpolation;
            self.interp_x = self.last_x + (self.next_x - self.last_x) * self.interpolation;
            self.interp_y = self.last_y + (self.next_y - self.last_y) * self.interpolation;
        }
    }
}

/// The [`Lerp`] to [`Transform` ] synchronization for predicted entities.
pub fn sync_predicted_sprites(
    mut query: Query<(&mut Lerp, &mut Transform), With<Predicted>>,
    client: Client,
) {
    for (mut interp, mut transform) in query.iter_mut() {
        interp.interpolate(client.client_interpolation().unwrap());

        transform.translation.x = interp.interp_x;
        transform.translation.y = interp.interp_y;
    }
}

/// The [`PhysicsWorld`] to [`Lerp`] synchronization for predicted entities.
pub fn sync_physics(
    mut physics_query: Query<(&mut Lerp, &PhysicsBodyHandle), With<Predicted>>,
    mut physics: ResMut<PhysicsWorld>,
) {
    for (mut lerp, handle) in physics_query.iter_mut() {
        let Some(rb) = physics.get_rigid_body_mut(handle.rigid_body) else { continue; };

        let pos = rb.translation() * 100.0;

        if vector![lerp.next_x, lerp.next_y].metric_distance(&vector![pos.x, pos.y]) > 0.5 {
            lerp.next_pos(pos.x, pos.y);
        }
    }
}

/// The [`PhysicsStateSync`] is to be synchronized with the [`PhysicsWorld`]
pub fn sync_physics_state(
    physics_query: Query<(&PhysicsStateSync, &PhysicsBodyHandle), With<Predicted>>,
    mut physics: ResMut<PhysicsWorld>,
) {
    for (state, handle) in physics_query.iter() {
        let Some(rb) = physics.get_rigid_body_mut(handle.rigid_body) else { continue; };

        if rb
            .translation()
            .metric_distance(&vector![*state.pos_x_m, *state.pos_y_m])
            > 0.5
        {
            rb.set_translation(vector![*state.pos_x_m, *state.pos_y_m], true);
            rb.set_linvel(vector![*state.linvel_x_m, *state.linvel_y_m], true);
        }
    }
}

pub fn sync_camera_pos(
    player_pos: Query<&Transform, With<Predicted>>,
    mut camera_pos: Query<&mut Transform, (With<Camera2d>, Without<Predicted>)>,
    owned: Res<OwnedEntities>,
) {
    if let Some(owned) = &owned.player_avatar {
        let Ok(player_transform) = player_pos.get(owned.predicted) else { return; };
        let mut camera_transform = camera_pos.single_mut();

        camera_transform.translation = player_transform.translation;
    }
}
