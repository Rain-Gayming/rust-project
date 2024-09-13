
use bevy::prelude::*;

use super::entities::*;

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
            speed: 15.0,
        }
    ));
}

pub fn move_player(
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