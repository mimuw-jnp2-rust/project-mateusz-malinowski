use crate::components::BackgroundTop;
use crate::{AppState, GameTextures, Movable, Velocity, WindowSize};
use bevy::prelude::*;

pub const BACKGROUND_SPRITE: &str = "background.png";
const BACKGROUND_SIZE: (f32, f32) = (256., 256.);

const SCROLLING_SPEED: f32 = 0.07;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, background_spawn_system)
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(background_scrolling_system),
            );
    }
}

fn background_spawn_system(mut commands: Commands, win_size: Res<WindowSize>) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., -(win_size.h + BACKGROUND_SIZE.1) / 2., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Movable {
            auto_despawn: false,
        })
        .insert(Velocity {
            x: 0.,
            y: -SCROLLING_SPEED,
        })
        .insert(BackgroundTop);
}

fn background_scrolling_system(
    mut commands: Commands,
    win_size: Res<WindowSize>,
    game_textures: Res<GameTextures>,
    mut query: Query<&mut Transform, With<BackgroundTop>>,
) {
    if let Ok(mut tf) = query.get_single_mut() {
        if tf.translation.y < (win_size.h + BACKGROUND_SIZE.1) / 2. {
            let mut spawn_tile = |offset: f32| {
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: game_textures.background.clone(),
                        transform: Transform {
                            translation: Vec3::new(offset, tf.translation.y, 0.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Movable { auto_despawn: true })
                    .insert(Velocity {
                        x: 0.,
                        y: -SCROLLING_SPEED,
                    });
            };

            spawn_tile(0.);

            let mut offset = BACKGROUND_SIZE.0;

            while offset < win_size.w / 2. {
                spawn_tile(offset);
                spawn_tile(-offset);
                offset += BACKGROUND_SIZE.0;
            }

            tf.translation.y += 256.;
        }
    }
}
