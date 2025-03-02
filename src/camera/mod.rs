use bevy::prelude::*;

use crate::AppState;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Target;

#[derive(Debug, Clone)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, movement.run_if(in_state(AppState::InGame)));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}

fn movement(
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<Target>)>,
    target: Query<&Transform, (With<Target>, Without<MainCamera>)>,
) {
    let Ok(mut transform) = camera.get_single_mut() else {
        return;
    };
    let Ok(target) = target.get_single() else {
        return;
    };

    let speed = 0.03;

    transform.translation = transform.translation.lerp(target.translation, speed);
}
