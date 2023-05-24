use bevy::prelude::Component;
use naia_bevy_shared::{Property, ProtocolPlugin, Replicate};

pub struct ComponentsPlugin;
impl ProtocolPlugin for ComponentsPlugin {
    fn build(&self, protocol: &mut naia_bevy_shared::Protocol) {
        protocol
            .add_component::<PhysicsStateSync>()
            .add_component::<CharacterEntity>()
            .add_component::<WallEntity>();
    }
}

/// Everything needed for extrapolation + absolute positions.
#[derive(Component, Replicate)]
pub struct PhysicsStateSync {
    pub linvel_x_m: Property<f32>,
    pub linvel_y_m: Property<f32>,
    pub angvel_rad: Property<f32>,

    pub pos_x_m: Property<f32>,
    pub pos_y_m: Property<f32>,
    pub ang_rad: Property<f32>,
}

// Tags

#[derive(Component, Replicate)]
pub struct CharacterEntity;

#[derive(Component, Replicate)]
pub struct WallEntity;
