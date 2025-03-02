use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::{player::Player, settings};

pub fn setup(
    mut commands: Commands,
    mut player_query: Query<Entity, With<Player>>,
    library: ResMut<AnimationLibrary>,
) {
    for entity in &mut player_query {
        commands
            .entity(entity)
            .insert(SpritesheetAnimation::from_id(
                library.animation_with_name("cat-sits").unwrap(),
            ));
    }
}

pub fn movement(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut KinematicCharacterController,
            &mut Sprite,
            &mut SpritesheetAnimation,
        ),
        With<Player>,
    >,
    library: ResMut<AnimationLibrary>,
) {
    for (mut controller, mut sprite, mut animation) in &mut query {
        let mut direction = Vec2::ZERO;
        if input.pressed(KeyCode::ArrowLeft) {
            sprite.flip_x = true;
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::ArrowRight) {
            sprite.flip_x = false;
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }

        if direction != Vec2::ZERO {
            if let Some(animation_id) = library.animation_with_name("cat-run") {
                if animation.animation_id != animation_id {
                    animation.switch(animation_id);
                }
            }
            controller.translation =
                Some(direction.normalize() * settings::cat::SPEED * time.delta_secs());
        } else if let Some(animation_id) = library.animation_with_name("cat-idle") {
            if animation.animation_id != animation_id {
                animation.switch(animation_id);
            }
        }
    }
}
