use bevy::{ecs::{query, system::SystemState}, prelude::*, transform};

mod entities;
use entities::player::*;


#[derive(Component)]
struct MyCameraMarker;

fn main() {   
    App::new()
        //plugins
        .add_plugins(DefaultPlugins)
        
        //startups
        .add_systems(Startup, (setup_camera, setup_player))

        //updates
        .add_systems(Update, update)
        .add_systems(Update, move_player)

        .run();
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