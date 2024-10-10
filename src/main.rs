use bevy::{prelude::*, utils::warn};

mod entities;
use collision::collider::*;
use entities::player::*;

mod inputs;
use inputs::keyboard::*;
use parallax::parallax::{move_parallax, ParallaxMarker};
use physics::physics::handle_physics;

mod collision;
mod parallax;
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
        .add_systems(Startup, (player_setup, spawn_camera, spawn_floor))
        //testing spawn_backgrounds
        .add_systems(Startup, spawn_backgrounds)
        //parallax
        .add_systems(Update, move_parallax)
        //physics
        .add_systems(
            Update,
            (handle_physics, entity_to_terrain_detection, collider_debug),
        )
        .run();
}

fn spawn_backgrounds(mut commands: Commands, asset_loader: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 15., -1.).with_scale(Vec3::new(5., 5., 5.)),
            texture: asset_loader.load("test_backgrounds_1.png"),
            ..default()
        },
        ParallaxMarker { layer: 1. },
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 15., -1.).with_scale(Vec3::new(5., 5., 5.)),
            texture: asset_loader.load("test_backgrounds_2.png"),
            ..default()
        },
        ParallaxMarker { layer: 2. },
    ));
}

fn spawn_floor(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., -15., 0.),
            ..default()
        },
        ColliderInfo {
            size_x: 50.,
            size_y: 50.,
            is_debug: true,
        },
    ));
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
