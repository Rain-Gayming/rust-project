use bevy::prelude::*;

use crate::collision::collider::*;
use crate::physics::physics::*;
use crate::KeyboardInputs;

use super::entity::*;

#[derive(Component)]
pub struct Player;

pub fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        EntityBundle {
            entity_info: EntityInformation { base_speed: 5.0 },
            sprite: SpriteBundle {
                texture: asset_server.load("test_player.png"),
                transform: Transform::from_scale(Vec3::splat(1.0))
                    .with_translation(Vec3::new(0., 0., 0.)),
                ..default()
            },
            physics_entity: PhysicsInformation { ..default() },
            collider: ColliderInfo {
                size_y: 5.0,
                size_x: 5.0,
                is_debug: true,
            },
        },
        Player,
    ));
}

pub fn handle_player_input(
    mut player_query: Query<(&mut PhysicsInformation, &mut EntityInformation), With<Player>>,
    keyboard_inputs: Res<KeyboardInputs>,
) {
    for (mut player, entity_info) in player_query.iter_mut() {
        player.currently_has_inputs = false;
        if keyboard_inputs.forwards {
            player.velocity.x = entity_info.base_speed;
            player.currently_has_inputs = true;
        }

        if keyboard_inputs.backwards {
            player.velocity.x = -entity_info.base_speed;
            player.currently_has_inputs = true;
        }
        if keyboard_inputs.up {
            player.velocity.y = entity_info.base_speed;
            player.currently_has_inputs = true;
        }

        if keyboard_inputs.down {
            player.velocity.y = -entity_info.base_speed;
            player.currently_has_inputs = true;
        }
    }
}
