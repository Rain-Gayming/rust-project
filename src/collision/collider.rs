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
            //is the terrains left hitting the terrains right?
            if entity_transform.translation.x - entity.size_x
                < terrain_transform.translation.x + terrain.size_x
                //is the entities right hitting the terrains left?
                && entity_transform.translation.x + entity.size_x
                    > terrain_transform.translation.x - terrain.size_x
                //is the entities bottom hitting the terrains top?
                && entity_transform.translation.y - entity.size_y
                    < terrain_transform.translation.y + terrain.size_y
                //is the entities top hitting the terrains bottom?
                && entity_transform.translation.y + entity.size_y
                    > terrain_transform.translation.y - terrain.size_y
            {
                //detect which side the player is closest to
                //push the player away from that side

                //top left
                //bottom right

                let x_offset = (entity_transform.translation.x
                    - (terrain.size_x + terrain_transform.translation.x))
                    + (terrain.size_x * 2.);
                let y_offset = (entity_transform.translation.y
                    - (terrain.size_y + terrain_transform.translation.y))
                    + (terrain.size_y * 2.);

                println!("x {}", x_offset);
                println!("y {}", y_offset);
                //if the entity is on the left side and is on the top
                if x_offset >= 0.0 && x_offset >= y_offset && y_offset != 0. {
                    println!("on the top of the right side");
                    entity_transform.translation.x += terrain.size_x;
                    return;
                }
                //if the entity is closer to the left than the top
                if x_offset <= 0.1 && x_offset <= y_offset && y_offset != 0. {
                    println!("on the top of the left side");
                    entity_transform.translation.x -= terrain.size_x;
                    return;
                }
                //if the entity is closer to the top than the right
                println!("pushing to the top");
                if y_offset > 0. {
                    entity_transform.translation.y += terrain.size_y;
                    return;
                } else {
                    entity_transform.translation.y -= terrain.size_y;
                    return;
                }
            }
        }
    }
}
