use bevy::prelude::*;

#[derive(Component)]
struct MyCameraMarker;

fn main() {   
    App::new()
        //plugins
        .add_plugins(DefaultPlugins)
        
        //startups
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_sprite)

        //updates
        .add_systems(Update, update)

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

fn setup_sprite(
    mut commands: Commands, 
    asset_server: Res<AssetServer>)
{   
    commands.spawn(SpriteBundle{
        texture: asset_server.load("test_sprite.png"),
        ..default()
    });
}
