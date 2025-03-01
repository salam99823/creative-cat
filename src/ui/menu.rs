use bevy::{color, prelude::*};

use crate::AppState;

#[derive(Resource, Debug)]
pub struct MenuData {
    pub button_entity: Entity,
}

pub fn handle(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &mut interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            next_state.set(AppState::InGame);
        }
    }
}

pub fn setup(mut commands: Commands) {
    let button_entity = commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.),
                        height: Val::Px(65.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::from(color::palettes::basic::BLACK)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Play"),
                        TextFont {
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });
        })
        .id();
    commands.insert_resource(MenuData { button_entity });
}

pub fn cleanup(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
}
