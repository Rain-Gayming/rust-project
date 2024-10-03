use bevy::prelude::*;

use crate::entities::player::*;
use crate::physics::physics::*;

// a resource that handles the inputs
#[derive(Resource, Default)]
pub struct KeyboardInputs {
    //movement inputs
    pub forwards: bool,
    pub backwards: bool,
    pub up: bool,
    pub down: bool,
    pub jump: bool,
}

pub fn handle_keyboard_inputs(
    mut keyboard_inputs: ResMut<KeyboardInputs>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut PhysicsInformation, With<Player>>,
) {
    //movement
    keyboard_inputs.forwards = keys.pressed(KeyCode::KeyD);
    keyboard_inputs.backwards = keys.pressed(KeyCode::KeyA);
    keyboard_inputs.up = keys.pressed(KeyCode::KeyW);
    keyboard_inputs.down = keys.pressed(KeyCode::KeyS);
    keyboard_inputs.jump = keys.pressed(KeyCode::Space);

    for mut player in player_query.iter_mut() {
        if !keyboard_inputs.forwards && !keyboard_inputs.backwards {
            player.velocity.x = 0.;
        }
        if !keyboard_inputs.up && !keyboard_inputs.down && !player.has_gravity {
            player.velocity.y = 0.;
        }
    }
}
