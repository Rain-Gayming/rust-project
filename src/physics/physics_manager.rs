use bevy::{animation::graph, prelude::*};

use crate::entities::entities::EntityValues;

#[derive(Component)]
pub struct PhysicsEntity{
    pub weight: f32,
    pub velocity: Vec2
}

pub fn physics_query(
    mut query: Query<(&mut Transform, &mut PhysicsEntity, &mut EntityValues)>
){
    //TODO: accelerate the gravity
    for (mut transform, physics_entity, entity_values) in query.iter_mut(){

        if !entity_values.is_grounded{
            //do gravity
            transform.translation.y -= 9.81 * physics_entity.weight;
        }

        transform.translation.y += physics_entity.velocity.y;       
        transform.translation.x += physics_entity.velocity.x;       
    }
}