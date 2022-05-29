use bevy::prelude::*;
use crate::{Fonts, ScoreText};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, score_display_system);
    }
}

fn score_display_system(
    mut commands: Commands,
    fonts: Res<Fonts>,
) {
    // Points counter
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
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
                    }, Default::default()
                ),
                ..default()
            })
                .insert(ScoreText);
        });
}