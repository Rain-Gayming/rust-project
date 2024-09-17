use bevy::{ecs::{query, system::SystemState}, prelude::*, transform::{self, commands}};

mod entities;
use entities::player::*;

mod physics;
use physics::*;
use physics_manager::physics_query;
use collisions::*;



#[derive(Component)]
struct MyCameraMarker;

fn main() {   
    App::new()
        //plugins
        .add_plugins(DefaultPlugins)
        

        //resources
        .insert_resource(KeyboardInputs{..default()})


        //startups
        .add_systems(Startup, (setup_camera, setup_player, spawn_floor))

        //updates
        .add_systems(Update, update)
        
        //physics
        .add_systems(Update, (physics_query, collision_query))

        //player
        .add_systems(Update, 
    (
                move_player,
                keyboard_input,
            )
        )

        .run();
}

fn spawn_floor(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    commands.spawn((
        SpriteBundle{
            texture: asset_server.load("grass.png"),
            transform: Transform::from_scale(Vec3::splat(1.)).with_translation(Vec3::new(0., 0., 0.,)),
            ..default()
        },
        Collider{
            size_x: 32.,
            size_y: 32.,
            collider_type: ColliderType::Cube
        }
    ));
}


fn update(){

}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            
            projection: OrthographicProjection{
                near: -1000.0,
                far: 1000.0,
                ..default()
            },
            
            ..default()
        },
        MyCameraMarker,
    ));
}