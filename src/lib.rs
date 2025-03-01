use bevy::app::PluginGroupBuilder;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use player::PlayerPlugin;
use settings::BACKGROUND_COLOR;
use ui::UiPlugin;

mod level;
mod player;
mod settings;
mod ui;

#[bevy_main]
fn main() {
    start();
}

#[derive(Debug)]
pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add(RapierDebugRenderPlugin::default())
            .add(TilemapPlugin)
            .add(SpritesheetAnimationPlugin::default())
            .add(PlayerPlugin)
            .add(UiPlugin)
    }
    fn set<T: Plugin>(self, plugin: T) -> PluginGroupBuilder {
        self.build().set(plugin)
    }
}

pub fn start() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Creative Cat".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F1)),
        ))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .init_state::<AppState>()
        .add_plugins(GamePlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}

fn setup(mut commands: Commands, mut configs: Query<&mut RapierConfiguration>) {
    commands.spawn(Camera2d);
    for mut configs in &mut configs {
        configs.gravity = Vec2::ZERO;
    }
}
