use crate::components::{ExitButton, MainMenu, NewGameButton};
use crate::{AppState, Fonts};
use bevy::app::AppExit;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, button_display_system)
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu)
                    .with_system(main_button_system)
                    .with_system(new_game_button_system)
                    .with_system(exit_button_system),
            )
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(show_menu))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(hide_menu));
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn main_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn new_game_button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<NewGameButton>)>,
    mut app_state: ResMut<State<AppState>>,
) {
    for (interaction) in interaction_query.iter() {
        if let Interaction::Clicked = *interaction {
            app_state.set(AppState::InGame).unwrap();
        }
    }
}

fn exit_button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction) in interaction_query.iter() {
        if let Interaction::Clicked = *interaction {
            exit.send(AppExit);
        }
    }
}

fn button_display_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fonts: Res<Fonts>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::rgb(0.04, 0.04, 0.04).into(),
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

                    parent.spawn_bundle(make_button()).with_children(|parent| {
                        parent.spawn_bundle(make_text("load", &fonts));
                    });

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

fn make_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(200.0), Val::Px(65.0)),
            // center button
            margin: Rect::all(Val::Auto),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        color: NORMAL_BUTTON.into(),
        ..default()
    }
}

fn make_text(text: &str, fonts: &Res<Fonts>) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            text,
            TextStyle {
                font: fonts.button.clone(),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
            Default::default(),
        ),
        ..default()
    }
}

fn show_menu(mut commands: Commands, mut query: Query<&mut Style, With<MainMenu>>) {
    for mut style in query.iter_mut() {
        style.display = Display::Flex;
    }
}

fn hide_menu(mut commands: Commands, mut query: Query<&mut Style, With<MainMenu>>) {
    for mut style in query.iter_mut() {
        style.display = Display::None;
    }
}
