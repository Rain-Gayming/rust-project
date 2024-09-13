use bevy::{ecs::system::SystemState, prelude::*};

#[derive(Component)]
struct MyCameraMarker;

#[derive(Component)]
struct KeyboardMovable;

fn main() {   
    App::new()
        //plugins
        .add_plugins(DefaultPlugins)
        
        //startups
        .add_systems(Startup, (setup_camera, setup_sprite))

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

fn setup_sprite(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
)
{ 
    commands.spawn((
        SpriteBundle{
            texture: asset_server.load("test_sprite.png"),
            transform: Transform::from_scale(Vec3::splat(1.)).with_translation(Vec3::new(0., 0., 0.,)),
            ..default()
        },
        KeyboardMovable
    ));
}

fn move_player(
    mut query: Query<&mut Transform, With<KeyboardMovable>>,
    keys: Res<ButtonInput<KeyCode>>,
) 
{
    if keys.just_pressed(KeyCode::KeyA){
        for mut transform in &mut query {
            transform.translation.x += 100.0;
        }
    }
}
