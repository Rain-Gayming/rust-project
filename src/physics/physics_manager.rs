use bevy::prelude::*;

#[derive(Component)]
pub struct PhysicsEntity{
    pub weight: f32,
    pub do_physics: bool,
    pub velocity: Vec2
}

pub fn physics_query(
    mut query: Query<(&mut Transform, &mut PhysicsEntity)>
){
    //TODO: accelerate the gravity
    for (mut transform, mut physics_entity) in query.iter_mut(){
        if physics_entity.do_physics{
            physics_entity.velocity.y -= 9.81 * physics_entity.weight;
        }
        transform.translation.y += physics_entity.velocity.y;       
    }
}