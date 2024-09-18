use bevy::prelude::*;

use crate::entities::{self, entities::EntityValues};

#[derive(Component)]
pub struct PhysicsEntity {
    pub weight: f32,
    pub velocity: Vec2,
}

pub fn physics_query(mut query: Query<(&mut Transform, &mut PhysicsEntity, &mut EntityValues)>) {
    //TODO: accelerate the gravity
    for (mut transform, mut physics_entity, entity_values) in query.iter_mut() {
        let gravity;

        if entity_values.is_grounded {
            gravity = 0.;
        } else {
            //do gravity
            if physics_entity.velocity.y != 0.0 {
                gravity = -9.81 * physics_entity.weight;
            } else {
                gravity = 0.;
            }
        }

        physics_entity.velocity.y = gravity;

        transform.translation.y += physics_entity.velocity.y;
        transform.translation.x += physics_entity.velocity.x;
    }
}
