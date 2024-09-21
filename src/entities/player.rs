use bevy::{math::vec2, prelude::*, reflect::hash_error};
use physics_manager::{add_force, PhysicsEntity};

use crate::physics;
use collisions::{Collider, ColliderType};
use physics::*;

use super::entities::*;

#[derive(Resource, Default)]
pub struct KeyboardInputs {
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub stuck: bool,
}

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("test_sprite.png"),
            transform: Transform::from_scale(Vec3::splat(1.))
                .with_translation(Vec3::new(0., 150., 0.)),
            ..default()
        },
        KeyboardMovable,
        EntityValues {
            speed: 5.0,
            jump_height: 60.0,
            is_grounded: false,
        },
        PhysicsEntity {
            weight: 1.,
            velocity: vec2(0., 0.),
            ..default()
        },
        Collider {
            size_x: 0.75,
            size_y: 1.5,
            collider_type: ColliderType::Cube,
            is_debug: true,
        },
    ));
}
pub fn move_player(
    mut query: Query<
        (&mut Transform, &mut EntityValues, &mut PhysicsEntity),
        With<KeyboardMovable>,
    >,
    keyboard_inputs: ResMut<KeyboardInputs>,
) {
    for (mut transform, mut entity_values, mut physics_entity) in query.iter_mut() {
        if keyboard_inputs.left {
            transform.translation.x -= entity_values.speed;
        }
        if keyboard_inputs.right {
            transform.translation.x += entity_values.speed;
        }

        if keyboard_inputs.jump && entity_values.is_grounded {
            //jump
            //physics_entity.has_external_forces = true;
            physics_entity.start_fall_point = transform.translation.y;
            //physics_entity.velocity.y += entity_values.jump_height;

            add_force(vec2(0., entity_values.jump_height), physics_entity.as_mut());
        }

        if keyboard_inputs.stuck {
            transform.translation = Vec3::new(0., 15., 0.);
            physics_entity.velocity = vec2(0., 0.);
        }

        //if the player has reached the top of their jump start applying gravity.
        if transform.translation.y >= (physics_entity.start_fall_point + entity_values.jump_height)
        {
            physics_entity.has_external_forces = false;
        }
    }
}

pub fn keyboard_input(
    mut keyboard_inputs: ResMut<KeyboardInputs>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    keyboard_inputs.left = keys.pressed(KeyCode::KeyA);

    keyboard_inputs.right = keys.pressed(KeyCode::KeyD);

    keyboard_inputs.jump = keys.just_pressed(KeyCode::Space);

    keyboard_inputs.stuck = keys.pressed(KeyCode::ControlLeft) && keys.pressed(KeyCode::KeyT);
}
