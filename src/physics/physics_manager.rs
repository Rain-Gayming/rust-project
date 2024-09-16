use bevy::{prelude::*};

use crate::entities;
use entities::*;

#[derive(Component)]
pub struct PhysicsEntity{
    pub weight: f32,
    pub do_physics: bool
}

pub fn physics_query(
    mut query: Query<(&mut Transform, &mut PhysicsEntity)>
){
    for (mut transform, mut physics_entity) in query.iter_mut(){
        if physics_entity.do_physics{
            transform.translation.y -= (9.81 * physics_entity.weight);
        }
    }
}