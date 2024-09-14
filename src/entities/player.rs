use bevy::{prelude::*};
use avian2d::{math::*, prelude::*};

use super::entities::*;


use crate::plugin;
use plugin::*;

#[derive(Resource, Default)]
pub struct KeyboardInputs{
    pub left: bool,
    pub right: bool,
}

pub fn setup_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
)
{ 
    commands.spawn((
        SpriteBundle{
            texture: asset_server.load("test_sprite.png"),
            transform: Transform::from_scale(Vec3::splat(1.)).with_translation(Vec3::new(0., 150., 0.,)),
            ..default()
        },
        KeyboardMovable,
        EntityValues{
            speed: 15.0,
        },
        RigidBody::Dynamic,
        Collider::capsule(0.5, 1.5)
    ));
}

pub fn move_player(
    mut query: Query<(&mut Transform, &mut EntityValues), With<KeyboardMovable>>,
    keyboard_inputs: Res<KeyboardInputs>,
) {
    for (mut transform, entity_values) in query.iter_mut() {
        if keyboard_inputs.left {
            transform.translation.x -= entity_values.speed;
        }
        if keyboard_inputs.right {
            transform.translation.x += entity_values.speed;
        }
    }
} 

pub fn keyboard_input(
    mut keyboard_inputs: ResMut<KeyboardInputs>,
    keys: Res<ButtonInput<KeyCode>>,
){

    if keys.pressed(KeyCode::KeyA) {
         keyboard_inputs.left = true;
    }else{
        keyboard_inputs.left = false;
    }

    if keys.pressed(KeyCode::KeyD) {
        keyboard_inputs.right = true;
    }else{
        keyboard_inputs.right = false;
    }
}