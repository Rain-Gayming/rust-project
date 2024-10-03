use bevy::prelude::*;

use crate::physics::physics::*;

#[derive(Bundle)]
pub struct EntityBundle {
    pub entity_info: EntityInformation,
    pub sprite: SpriteBundle,
    pub physics_entity: PhysicsInformation,
}

#[derive(Component)]
pub struct EntityInformation {
    pub base_speed: f32,
}
