use bevy::prelude::*;

use crate::{collision::collider::ColliderInfo, physics::physics::*};

#[derive(Bundle)]
pub struct EntityBundle {
    pub entity_info: EntityInformation,
    pub sprite: SpriteBundle,
    pub physics_entity: PhysicsInformation,
    pub collider: ColliderInfo,
}

#[derive(Component)]
pub struct EntityInformation {
    pub base_speed: f32,
}
