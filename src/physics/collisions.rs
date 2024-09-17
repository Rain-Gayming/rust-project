use bevy::prelude::*;
use std::fmt;

use crate::entities::{self, entities::EntityValues};
use entities::*;

use super::physics_manager::PhysicsEntity;

#[derive(Component)]
pub struct Collider{
    pub size_x: f32,
    pub size_y: f32,
    pub collider_type: ColliderType
}

pub enum ColliderType{
    _None,
    Cube,
    _OneSided,
}

pub fn collision_query(
    mut entity_query: Query<(&mut Transform, &mut Collider, &mut PhysicsEntity, &mut EntityValues), With<EntityValues>>,
    mut terrain_query: Query<(&mut Transform, &mut Collider), Without<EntityValues>>
){
    //checks for each entity and terrain collider
    for (mut e_transform, e_collider, mut e_physics, mut entity_values) in entity_query.iter_mut(){
        for (t_tranform, t_collider) in terrain_query.iter_mut(){
            
            
            //vertical collision
            if e_transform.translation.y < t_tranform.translation.y + t_collider.size_y
            && e_transform.translation.y + e_collider.size_y > t_tranform.translation.y
            && e_transform.translation.x < t_tranform.translation.x + t_collider.size_x
            && e_transform.translation.x  + e_collider.size_x > t_tranform.translation.x
            {
                // gets the difference in x and y positions
                let x_offset = e_transform.translation.x - t_tranform.translation.x;
                let y_offset = e_transform.translation.y + t_tranform.translation.y;

                e_transform.translation.y -= y_offset;
                e_transform.translation.x -= x_offset;

                e_physics.do_gravity = false;
                entity_values.is_grounded = true;
                
            }else{
                e_physics.do_gravity = true;
                entity_values.is_grounded = false;
            }
        }
    }
}

