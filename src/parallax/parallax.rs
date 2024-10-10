use bevy::prelude::*;

use crate::{
    entities::{entity::EntityInformation, player::*},
    physics::physics::*,
};

#[derive(Component)]
pub struct ParallaxMarker {
    pub layer: f32,
}

pub fn move_parallax(
    mut backgrounds: Query<(&mut Transform, &mut ParallaxMarker)>,
    player: Query<
        (&mut Transform, &mut Player, &mut PhysicsInformation),
        (With<EntityInformation>, Without<ParallaxMarker>),
    >,
) {
    for (mut background_transform, parallax) in backgrounds.iter_mut() {
        for (player_transform, _player_component, player_physics) in player.iter() {
            if player_physics.velocity.x != 0. {
                background_transform.translation.x =
                    (player_transform.translation.x / 1.5) / parallax.layer;
            }
        }
    }
}
