use bevy::prelude::*;

mod entities;
use entities::player::*;

mod inputs;
use inputs::keyboard::*;
use physics::physics::handle_physics;

mod physics;

#[derive(Component)]
pub struct PlayerCamera;

fn main() {
    App::new()
        .insert_resource(KeyboardInputs { ..default() })
        .add_plugins(DefaultPlugins)
        .add_systems(Update, handle_keyboard_inputs)
        //player
        .add_systems(Update, handle_player_input)
        .add_systems(Startup, (player_setup, spawn_camera))
        //physics
        .add_systems(Update, handle_physics)
        .run();
}

// oct 10th 2024: spawns the players camera
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            projection: OrthographicProjection {
                near: 1000.,
                far: -1000.,
                ..default()
            },

            ..default()
        },
        PlayerCamera,
    ));
}
