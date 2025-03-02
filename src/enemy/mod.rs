use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use rand::Rng;

use crate::AppState;

mod movement;

#[derive(Component)]
pub struct Enemy;

#[derive(Debug, Clone)]
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(
                Update,
                movement::move_enemy.run_if(in_state(AppState::InGame)),
            );
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut library: ResMut<AnimationLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut rng = rand::rng();
    let spritesheet = Spritesheet::new(8, 1);
    let clip = Clip::from_frames(spritesheet.row(0));
    let clip_id = library.register_clip(clip);
    let animation = Animation::from_clip(clip_id);

    let sprite = Sprite::from_atlas_image(
        assets.load("Bat.png"),
        TextureAtlas {
            layout: atlas_layouts.add(spritesheet.atlas_layout(225, 150)),
            ..default()
        },
    );
    for _ in 0..15 {
        let (mut x, mut y) = (
            (rng.random_range(1..10) * 50) as f32,
            (rng.random_range(1..10) * 50) as f32,
        );
        if rng.random_bool(0.5) {
            x = -x;
        }
        if rng.random_bool(0.5) {
            y = -y;
        }
        let animation_id = library.register_animation(
            animation.with_duration(AnimationDuration::PerFrame(rng.random_range(100..150))),
        );

        commands.spawn((
            Enemy,
            KinematicCharacterController::default(),
            sprite.clone(),
            LockedAxes::ROTATION_LOCKED,
            Transform::from_xyz(x, y, 0.1).with_scale(Vec3::new(0.18, 0.18, 1.)),
            Collider::ball(35.),
            RigidBody::KinematicPositionBased,
            SpritesheetAnimation::from_id(animation_id),
        ));
    }
}
