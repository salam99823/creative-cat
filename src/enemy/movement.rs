use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::Player;

use super::Enemy;

pub fn move_enemy(
    time: Res<Time>,
    mut enemy: Query<(&mut KinematicCharacterController, &Transform, &mut Sprite), With<Enemy>>,
    player: Query<&Transform, With<Player>>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };
    for (mut controller, transform, mut sprite) in &mut enemy {
        let delta = (player.translation - transform.translation).xy();
        sprite.flip_x = delta.x < 0.;
        if delta.distance(Vec2::ZERO) > 35. {
            controller.translation = delta.try_normalize().map(|v| v * 100. * time.delta_secs());
        }
    }
}
