use crate::ui::buttons::{hide, make_button, make_text, show, ButtonsPlugin};
use crate::ui::components::{ExitButton, LoadButton, MainMenu, NewGameButton};
use crate::{AppState, Fonts};
use bevy::app::AppExit;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, button_display_system)
            .add_plugin(ButtonsPlugin)
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu)
                    .with_system(new_game_button_system)
                    .with_system(load_button_system)
                    .with_system(exit_button_system),
            )
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(show::<MainMenu>))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(hide::<MainMenu>));
    }
}

fn new_game_button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<NewGameButton>)>,
    mut app_state: ResMut<State<AppState>>,
) {
    for interaction in interaction_query.iter() {
        if let Interaction::Clicked = *interaction {
            app_state.set(AppState::InitNewGame).unwrap();
        }
    }
}

fn load_button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<LoadButton>)>,
    mut app_state: ResMut<State<AppState>>,
) {
    for interaction in interaction_query.iter() {
        if let Interaction::Clicked = *interaction {
            app_state.set(AppState::Load).unwrap();
        }
    }
}

fn exit_button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in interaction_query.iter() {
        if let Interaction::Clicked = *interaction {
            exit.send(AppExit);
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
                ..default()
            },
            color: Color::rgb(0.06, 0.06, 0.06).into(),
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
                            parent.spawn_bundle(make_text("new game", &fonts));
                        })
                        .insert(NewGameButton);

                    parent
                        .spawn_bundle(make_button())
                        .with_children(|parent| {
                            parent.spawn_bundle(make_text("load game", &fonts));
                        })
                        .insert(LoadButton);

                    parent
                        .spawn_bundle(make_button())
                        .with_children(|parent| {
                            parent.spawn_bundle(make_text("exit", &fonts));
                        })
                        .insert(ExitButton);
                });
        })
        .insert(MainMenu);
}
