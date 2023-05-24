use bevy::prelude::*;
use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct PhysicsBodyHandle {
    pub rigid_body: RigidBodyHandle,
    pub collider: ColliderHandle,
}
