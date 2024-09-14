use bevy::{prelude::*};
use avian2d::{math::*, prelude::*};

use super::entities::*;


use crate::plugin;
use plugin::*;

#[derive(Component, Default)]
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
        KeyboardInputs{
            ..default()
        }, 
        EntityValues{
            speed: 15.0,
        },
        CharacterControllerBundle::new(Collider::capsule(12.5, 20.0), Vector::NEG_Y * 1500.0),
    ));
}

pub fn move_player(
    mut query: Query<(&mut Transform, &mut EntityValues, &mut KeyboardInputs), With<KeyboardMovable>>,
) {
    for (mut transform, entity_values, keyboard_inputs) in query.iter_mut() {
        if keyboard_inputs.left {
            transform.translation.x -= entity_values.speed;
        }
        if keyboard_inputs.right {
            transform.translation.x += entity_values.speed;
        }
    }
} 

pub fn keyboard_input(
    mut query: Query<(&mut KeyboardInputs)>,
    keys: Res<ButtonInput<KeyCode>>,
){

    for mut keyboard_inputs in query.iter_mut() {
        
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

}