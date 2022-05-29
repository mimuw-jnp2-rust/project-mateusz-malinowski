use crate::{AppState, Fonts};
use bevy::prelude::*;

pub struct ButtonsPlugin;

impl Plugin for ButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(buttons_system))
            .add_system_set(SystemSet::on_update(AppState::Paused).with_system(buttons_system));
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn buttons_system(
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

pub fn make_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(210.0), Val::Px(65.0)),
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

pub fn make_text(text: &str, fonts: &Res<Fonts>) -> TextBundle {
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

pub fn show<T: Component>(mut query: Query<&mut Style, With<T>>) {
    for mut style in query.iter_mut() {
        style.display = Display::Flex;
    }
}

pub fn hide<T: Component>(mut query: Query<&mut Style, With<T>>) {
    for mut style in query.iter_mut() {
        style.display = Display::None;
    }
}
