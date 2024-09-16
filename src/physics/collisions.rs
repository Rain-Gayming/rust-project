use bevy::prelude::*;

use crate::entities::{self, entities::EntityValues};
use entities::*;

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
    mut entity_query: Query<(&mut Transform, &mut Collider), With<EntityValues>>,
    mut terrain_query: Query<(&mut Transform, &mut Collider), Without<EntityValues>>
){
    for (e_tranform, e_collider) in entity_query.iter_mut(){
        for (t_tranform, t_collider) in terrain_query.iter_mut(){
            if 
                   e_tranform.translation.x < t_tranform.translation.x + t_collider.size_x
                && e_tranform.translation.x  + e_collider.size_x > t_tranform.translation.x
                && e_tranform.translation.y < t_tranform.translation.y + t_collider.size_y
                && e_tranform.translation.y + t_collider.size_y > t_tranform.translation.y
            {
                println!("collision detected");
            }
        }
    }
}

