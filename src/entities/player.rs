use bevy::{prelude::*};
use physics_manager::PhysicsEntity;

use crate::physics;
use physics::*;

use super::entities::*;


#[derive(Resource, Default)]
pub struct KeyboardInputs{
    pub left: bool,
    pub right: bool,
    pub jump: bool,
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
            speed: 5.0,
            jump_height: 350.0,
            is_grounded: false,
        },
        PhysicsEntity{
            weight: 1.0,
            do_physics: true
        }
    ));
}  
pub fn move_player(
    mut query: Query<(&mut Transform, &mut EntityValues), With<KeyboardMovable>>,
    mut keyboard_inputs: ResMut<KeyboardInputs>,
) {
    for (mut transform, entity_values) in query.iter_mut() {
        if keyboard_inputs.left {
            transform.translation.x -= entity_values.speed;
        }
        if keyboard_inputs.right {
            transform.translation.x += entity_values.speed;
        }

        if keyboard_inputs.jump{
            //jump
            keyboard_inputs.jump = false
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

    if keys.just_pressed(KeyCode::Space) {
        keyboard_inputs.jump = true;
    }else{
        keyboard_inputs.jump = false;
    }
}