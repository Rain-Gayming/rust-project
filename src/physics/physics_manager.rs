use bevy::{animation::graph, prelude::*};

#[derive(Component)]
pub struct PhysicsEntity{
    pub weight: f32,
    pub do_physics: bool,
    pub do_gravity: bool,
    pub velocity: Vec2
}

pub fn physics_query(
    mut query: Query<(&mut Transform, &mut PhysicsEntity)>
){
    //TODO: accelerate the gravity
    for (mut transform, mut physics_entity) in query.iter_mut(){
        println!("{}", physics_entity.velocity.y);
        let gravity;
        if physics_entity.do_gravity{
            gravity = 9.81 * physics_entity.weight;
            physics_entity.velocity.y -= gravity;
        }else{
            gravity = 0.;
            physics_entity.velocity.y -= gravity;
        }
        transform.translation.y += physics_entity.velocity.y;       
    }
}