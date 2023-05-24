use naia_bevy_shared::{EntityProperty, Message, ProtocolPlugin};

pub struct MessagesPlugin;
impl ProtocolPlugin for MessagesPlugin {
    fn build(&self, protocol: &mut naia_bevy_shared::Protocol) {
        protocol
            .add_message::<Auth>()
            .add_message::<PlayerInput>()
            .add_message::<EntityAssignment>()
            .add_message::<NewTarget>();
    }
}

#[derive(Message)]
pub struct Auth {
    pub name: String,
    pub channel_password: String,
}

#[derive(Message)]
pub struct PlayerInput {
    pub entity: EntityProperty,

    pub x_axis: f32,
    pub y_axis: f32,
}

impl PlayerInput {
    pub fn from_wasd(w: bool, a: bool, s: bool, d: bool) -> Self {
        let y_axis = match (w, s) {
            (true, true) | (false, false) => 0.0,
            (true, false) => 1.0,
            (false, true) => -1.0,
        };
        let x_axis = match (d, a) {
            (true, true) | (false, false) => 0.0,
            (true, false) => 1.0,
            (false, true) => -1.0,
        };

        Self {
            entity: EntityProperty::new(),
            x_axis,
            y_axis,
        }
    }

    pub fn from_axes(x: f32, y: f32) -> Self {
        Self {
            entity: EntityProperty::new(),
            x_axis: x,
            y_axis: y,
        }
    }
}

/// How the player knows which character they control.
#[derive(Message)]
pub struct EntityAssignment {
    pub entity: EntityProperty,
    pub assign: bool,
}

impl EntityAssignment {
    pub fn new(assign: bool) -> Self {
        Self {
            entity: EntityProperty::new(),
            assign,
        }
    }
}

#[derive(Message)]
pub struct NewTarget {
    pub entity: EntityProperty,
}

impl NewTarget {
    pub fn new() -> Self {
        Self {
            entity: EntityProperty::new(),
        }
    }
}
