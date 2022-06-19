use crate::ui::buttons::{hide, make_button, make_text, show, ButtonsPlugin};
use crate::ui::components::{ContinueButton, MainMenuButton, PauseMenu, SaveButton};
use crate::{AppState, Fonts};
use bevy::prelude::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, button_display_system)
            .add_plugin(ButtonsPlugin)
            .add_system_set(
                SystemSet::on_update(AppState::Paused)
                    .with_system(continue_button_system)
                    .with_system(save_button_system)
                    .with_system(main_menu_button_system),
            )
            .add_system_set(SystemSet::on_enter(AppState::Paused).with_system(show::<PauseMenu>))
            .add_system_set(SystemSet::on_exit(AppState::Paused).with_system(hide::<PauseMenu>));
    }
}

fn continue_button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ContinueButton>)>,
    mut app_state: ResMut<State<AppState>>,
) {
    for interaction in interaction_query.iter() {
        if let Interaction::Clicked = *interaction {
            app_state.set(AppState::InGame).unwrap();
        }
    }
}

fn save_button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<SaveButton>)>,
    mut app_state: ResMut<State<AppState>>,
) {
    for interaction in interaction_query.iter() {
        if let Interaction::Clicked = *interaction {
            app_state.set(AppState::Save).unwrap();
        }
    }
}

fn main_menu_button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
    mut app_state: ResMut<State<AppState>>,
) {
    for interaction in interaction_query.iter() {
        if let Interaction::Clicked = *interaction {
            app_state.set(AppState::MainMenu).unwrap();
        }
    }
}

fn button_display_system(mut commands: Commands, fonts: Res<Fonts>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                display: Display::None,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(make_button())
                        .with_children(|parent| {
                            parent.spawn_bundle(make_text("continue", &fonts));
                        })
                        .insert(ContinueButton);

                    parent
                        .spawn_bundle(make_button())
                        .with_children(|parent| {
                            parent.spawn_bundle(make_text("save game", &fonts));
                        })
                        .insert(SaveButton);

                    parent
                        .spawn_bundle(make_button())
                        .with_children(|parent| {
                            parent.spawn_bundle(make_text("main menu", &fonts));
                        })
                        .insert(MainMenuButton);
                });
        })
        .insert(PauseMenu);
}
