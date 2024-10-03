use bevy::prelude::*;

use crate::entities::player;

#[derive(Component, Default)]
pub struct PhysicsInformation {
    pub velocity: Vec2,
    pub currently_has_inputs: bool,
    pub has_gravity: bool,
}

pub fn handle_physics(mut physics_query: Query<(&mut Transform, &mut PhysicsInformation)>) {
    for (mut physics_entity, mut physics_info) in physics_query.iter_mut() {
        if physics_info.has_gravity {
            physics_info.velocity.y = -9.81;
        }

        physics_entity.translation.x += physics_info.velocity.x;
        physics_entity.translation.y += physics_info.velocity.y;
    }
}
