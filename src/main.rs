use bevy::{ecs::{query, system::SystemState}, prelude::*, transform};
use avian2d::{math::*, prelude::*};
use bevy_asset_loader::prelude::*;

mod entities;
use entities::player::*;

mod plugin;
use plugin::*;


#[derive(Component)]
struct MyCameraMarker;

fn main() {   
    App::new()
        //plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        
        //startups
        .add_systems(Startup, (setup_camera, setup_player))

        //updates
        .add_systems(Update, update)

        //player
        .add_systems(Update, 
    (
                move_player,
                keyboard_input,
            )
        )

        //resources
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))

        //asset loader
        .init_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .load_collection::<AudioAssets>(),
        )
        .add_systems(OnEnter(MyStates::Next), start_background_audio)

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

#[derive(AssetCollection, Resource)]
struct AudioAssets {
    #[asset(path = "audio/background.ogg")]
    background: Handle<AudioSource>,
}

/// This system runs in MyStates::Next. Thus, AudioAssets is available as a resource
/// and the contained handle is done loading.
fn start_background_audio(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn(AudioBundle {
        source: audio_assets.background.clone(),
        settings: PlaybackSettings::LOOP,
    });
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
}