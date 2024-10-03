use bevy::prelude::*;

use crate::entities::entity::EntityInformation;

#[derive(Component)]
pub struct ColliderInfo {
    pub size_y: f32,
    pub size_x: f32,
    pub is_debug: bool,
}

#[derive(Component)]
pub struct DebugColliderBundle;

pub fn collider_debug(
    mut collider_query: Query<(&mut Transform, &mut ColliderInfo)>,
    render_query: Query<Entity, With<DebugColliderBundle>>,
    mut commands: Commands,
) {
    // removes all previously rendered collision boxes
    for entity in render_query.iter() {
        commands.entity(entity).remove::<SpriteBundle>();
    }

    for (c_transform, collider) in collider_query.iter_mut() {
        //top left
        if collider.is_debug {
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
}

pub fn entity_to_terrain_detection(
    mut entity_query: Query<(&mut ColliderInfo, &mut Transform), With<EntityInformation>>,
    mut terrain_query: Query<(&mut ColliderInfo, &mut Transform), Without<EntityInformation>>,
) {
    for (entity, mut entity_transform) in entity_query.iter_mut() {
        for (terrain, mut terrain_transform) in terrain_query.iter_mut() {
            if entity_transform.translation.x - entity.size_x
                < terrain_transform.translation.x + terrain.size_x
                && entity_transform.translation.x + entity.size_x
                    > terrain_transform.translation.x - terrain.size_x
                && entity_transform.translation.y - entity.size_y
                    < terrain_transform.translation.y + terrain.size_y
                && entity_transform.translation.y + entity.size_y
                    > terrain_transform.translation.y - terrain.size_y
            {
                //works only for collidingwith the top of the terrain
                //make it so it checks which way youre colliding
                let x_offset = entity_transform.translation.x + terrain_transform.translation.x;
                let y_offset = entity_transform.translation.y + terrain_transform.translation.y;

                entity_transform.translation.x -= x_offset;
                entity_transform.translation.y -= y_offset;

                println!("collision detected");
            }
        }
    }
}
