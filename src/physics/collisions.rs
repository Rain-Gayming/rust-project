use bevy::{prelude::*, scene::ron::de};

use crate::entities::entities::EntityValues;

use super::physics_manager::PhysicsEntity;

#[derive(Component)]
pub struct Collider {
    pub size_x: f32,
    pub size_y: f32,
    pub collider_type: ColliderType,
    pub is_debug: bool,
}

#[derive(Component)]
pub struct DebugColliderBundle;

#[derive(PartialEq)]
pub enum ColliderType {
    None,
    Cube,
    _OneSided,
}

//unused
//is supposed to render the corners of the collider

pub fn collider_debug(
    mut collider_query: Query<(&mut Transform, &mut Collider)>,
    mut render_query: Query<Entity, With<DebugColliderBundle>>,
    mut commands: Commands,
) {
    // removes all previously rendered collision boxes
    for entity in render_query.iter() {
        commands.entity(entity).remove::<SpriteBundle>();
    }

    for (c_transform, collider) in collider_query.iter_mut() {
        //top left
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    c_transform.translation.x - collider.size_x,
                    c_transform.translation.y + collider.size_y,
                    1.0,
                ),
                ..default()
            },
            DebugColliderBundle,
        ));
        //top right
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    c_transform.translation.x + collider.size_x,
                    c_transform.translation.y + collider.size_y,
                    1.0,
                ),
                ..default()
            },
            DebugColliderBundle,
        ));
        //bottom left
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    c_transform.translation.x - collider.size_x,
                    c_transform.translation.y - collider.size_y,
                    1.0,
                ),
                ..default()
            },
            DebugColliderBundle,
        ));
        //bottom right
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    c_transform.translation.x + collider.size_x,
                    c_transform.translation.y - collider.size_y,
                    1.0,
                ),
                ..default()
            },
            DebugColliderBundle,
        ));
    }
}

pub fn collision_query(
    mut entity_query: Query<
        (
            &mut Transform,
            &mut Collider,
            &mut PhysicsEntity,
            &mut EntityValues,
        ),
        With<EntityValues>,
    >,
    mut terrain_query: Query<(&mut Transform, &mut Collider), Without<EntityValues>>,
) {
    // checks for each entity and terrain collider
    for (mut e_transform, e_collider, e_physics, mut entity_values) in entity_query.iter_mut() {
        for (t_transform, t_collider) in terrain_query.iter_mut() {
            if e_collider.collider_type != ColliderType::None {
                // aabb try 1
                // should check if an entity is inside of the terrain
                // outside of combat entities should never collider with each other
                if e_transform.translation.y <= t_transform.translation.y + t_collider.size_y
                    && e_transform.translation.y + e_collider.size_y
                        >= t_transform.translation.y - t_collider.size_y
                    && e_transform.translation.x <= t_transform.translation.x + t_collider.size_x
                    && e_transform.translation.x + e_collider.size_x
                        >= t_transform.translation.x - t_collider.size_x
                {
                    println!("in a collider");
                    // gets the difference in x and y position
                    let x_offset = t_transform.translation.x + e_transform.translation.x;
                    let y_offset = t_transform.translation.y + e_transform.translation.y;

                    // moves the player based on the difference
                    e_transform.translation.y += y_offset;
                    e_transform.translation.x += x_offset;

                    //entity_values.is_grounded = true;
                } else {
                    entity_values.is_grounded = false;
                }
            }
        }
    }
}
