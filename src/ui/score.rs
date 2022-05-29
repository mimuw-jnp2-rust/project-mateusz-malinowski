use crate::{AppState, Fonts, Score, ScoreText};
use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, score_display_system)
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(score_update_system),
            );
    }
}

fn score_display_system(mut commands: Commands, fonts: Res<Fonts>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(15.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                margin: Rect::all(Val::Px(5.0)),
                ..default()
            },
            text: Text::with_section(
                "0",
                TextStyle {
                    font: fonts.score.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..default()
        })
        .insert(ScoreText);
}

fn score_update_system(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = score.0.to_string();
    }
}
