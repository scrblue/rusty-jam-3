//! This module contains logic related to spawning a given "entity type" based on the insertion of
//! a marker tag defined in `shared`.

use bevy::prelude::*;
use naia_bevy_client::{events::InsertComponentEvents, CommandsExt};
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder};
use shared::{
    components::CharacterEntity,
    physics::{components::PhysicsBodyHandle, Layer, PhysicsWorld},
};

use crate::in_game::{sync::Lerp, Confirmed, Predicted};

pub fn insert_character_to_world(physics: &mut PhysicsWorld, layer: Layer) -> PhysicsBodyHandle {
    let rb = RigidBodyBuilder::dynamic().linear_damping(1.0).build();
    let cl = ColliderBuilder::cuboid(0.5, 0.5).collision_groups(layer.into());

    let (rigid_body, collider) = physics.insert(rb, cl);

    PhysicsBodyHandle {
        rigid_body,
        collider,
    }
}

/// Listens for the insertion of [`CharacterEntity`] components from the server. If one is inserted,
/// that means a new character must be spawned.
pub fn listen_character_creation(
    mut reader: EventReader<InsertComponentEvents>,
    mut physics: ResMut<PhysicsWorld>,
    mut commands: Commands,
) {
    for event in reader.iter() {
        for entity in event.read::<CharacterEntity>() {
            let handle = insert_character_to_world(&mut physics, Layer::Predicted);
            let predicted = commands
                .entity(entity)
                .duplicate()
                .insert(handle)
                .insert(Lerp::new(0.0, 0.0))
                .insert(SpriteBundle {
                    sprite: Sprite {
                        color: Color::FUCHSIA,
                        custom_size: Some(Vec2::new(100.0, 100.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..Default::default()
                })
                .insert(Predicted)
                .id();

            commands.entity(entity).insert(Confirmed(predicted));
        }
    }
}
