use bevy::{prelude::*, utils::HashMap};
use lazy_static::lazy_static;
use naia_bevy_shared::Tick;
use rapier2d::prelude::*;

pub mod components;
pub mod systems;

#[derive(Resource)]
pub struct PhysicsWorld {
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    ccd_solver: CCDSolver,

    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,

    physic_pipeline: PhysicsPipeline,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self {
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            ccd_solver: CCDSolver::new(),

            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),

            physic_pipeline: PhysicsPipeline::new(),
        }
    }
}

impl PhysicsWorld {
    pub fn step(&mut self) {
        self.physic_pipeline.step(
            &vector![0.0, 0.0],
            &INTEGRATION_PARAMETERS,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            &(),
            &(),
        )
    }

    pub fn step_back(&mut self) {
        self.physic_pipeline.step(
            &vector![0.0, 0.0],
            &REV_INTEGRATION_PARAMETERS,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            &(),
            &(),
        )
    }

    pub fn insert(
        &mut self,
        rb: impl Into<RigidBody>,
        col: impl Into<Collider>,
    ) -> (RigidBodyHandle, ColliderHandle) {
        let rb = self.rigid_body_set.insert(rb);
        let col = self
            .collider_set
            .insert_with_parent(col, rb, &mut self.rigid_body_set);
        (rb, col)
    }

    pub fn get_rigid_body_mut(&mut self, handle: RigidBodyHandle) -> Option<&mut RigidBody> {
        self.rigid_body_set.get_mut(handle)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Layer {
    Confirmed,
    Predicted,
    Static,
}

impl Into<InteractionGroups> for Layer {
    fn into(self) -> InteractionGroups {
        match self {
            Layer::Confirmed => InteractionGroups::new(Group::GROUP_1, Group::GROUP_1),
            Layer::Predicted => InteractionGroups::new(Group::GROUP_2, Group::GROUP_2),
            Layer::Static => InteractionGroups::all(),
        }
    }
}

lazy_static! {
    pub static ref INTEGRATION_PARAMETERS: IntegrationParameters = IntegrationParameters {
        dt: 1.0 / 20.0,
        ..Default::default()
    };
    pub static ref REV_INTEGRATION_PARAMETERS: IntegrationParameters = IntegrationParameters {
        dt: -1.0 / 20.0,
        ..Default::default()
    };
}
