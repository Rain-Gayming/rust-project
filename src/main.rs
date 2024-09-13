use bevy::{ecs::{query, system::SystemState}, prelude::*, transform};

mod player;


#[derive(Component)]
struct MyCameraMarker;

#[derive(Component)]
struct KeyboardMovable;

#[derive(Component)]
struct EntityValues{
    speed: f32,
}

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

fn setup_player(
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
        }
    ));
}

fn move_player(
    mut query: Query<(&mut Transform, &mut EntityValues), With<KeyboardMovable>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut transform, entity_values) in query.iter_mut() {
        if keys.just_pressed(KeyCode::KeyA) {
            transform.translation.x -= entity_values.speed;
        }
        if keys.just_pressed(KeyCode::KeyD) {
            transform.translation.x += entity_values.speed;
        }
    }
} 