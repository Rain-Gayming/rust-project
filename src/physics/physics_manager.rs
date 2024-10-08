use bevy::prelude::*;

use crate::entities::entities::EntityValues;

#[derive(Component, Default)]
pub struct PhysicsEntity {
    pub weight: f32,
    pub velocity: Vec2,
    pub has_external_forces: bool,
    pub start_fall_point: f32,
}

pub fn physics_query(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PhysicsEntity, &mut EntityValues)>,
) {
    //TODO: accelerate the gravitiy
    //positive forces less than 60 dont seem to be applied
    for (mut transform, mut physics_entity, entity_values) in query.iter_mut() {
        transform.translation.y += physics_entity.velocity.y;
        transform.translation.x += physics_entity.velocity.x;

        let gravity;

        if !entity_values.is_grounded && !physics_entity.has_external_forces {
            //gravity = -9.81 * physics_entity.weight;
            gravity = 0.;
        } else {
            gravity = 0.;
        }

        physics_entity.velocity.y += gravity * time.delta_seconds()
    }
}

pub fn add_force(force: Vec2, entity: &mut PhysicsEntity) {
    entity.has_external_forces = true;
    entity.velocity += force;

    println!("x: {}, y: {}", entity.velocity.x, entity.velocity.y);
}
