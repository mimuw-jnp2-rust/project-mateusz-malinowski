use crate::components::{FromPlayer, Movable, Player, SpriteSize, Velocity};
use crate::{AppState, GameTextures, Laser, WindowSize, SPRITE_SCALE};
use bevy::prelude::*;
use std::f32::consts::PI;

// Asset Constants
pub const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);

// pub const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
// const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

pub const PLAYER_LASER_SPRITE: &str = "laserGreen12.png";
const PLAYER_LASER_SIZE: (f32, f32) = (13., 37.);

// pub const PLAYER_LASER_SPRITE: &str = "laserGreenRound.png";
// const PLAYER_LASER_SIZE: (f32, f32) = (20., 20.);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(player_fire_system2)
                    .with_system(player_keyboard_event_system),
            );
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    window_size: ResMut<WindowSize>,
) {
    let bottom = -window_size.h / 2.;

    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + PLAYER_SIZE.1 * SPRITE_SCALE / 2., 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(Movable {
            auto_despawn: false,
        })
        .insert(Velocity { x: 0., y: 0. })
        .insert(SpriteSize::from(PLAYER_SIZE));
}

fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);
            let x_offset = PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 5.;

            let mut spawn_laser = |x_offset: f32| {
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: game_textures.player_laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(x + x_offset, y + 15., 0.),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Movable { auto_despawn: true })
                    .insert(Velocity { x: 0., y: 1. })
                    .insert(Laser)
                    .insert(FromPlayer)
                    .insert(SpriteSize::from(PLAYER_LASER_SIZE));
            };

            spawn_laser(x_offset);
            spawn_laser(-x_offset);
        }
    }
}

fn player_fire_system2(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);
            // let x_offset = PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 5.;

            let mut spawn_laser = |angle: f32| {
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: game_textures.player_laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(x, y + 15., 0.),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                            rotation: Quat::from_rotation_z(-angle),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Movable { auto_despawn: true })
                    .insert(Velocity {
                        x: angle.sin(),
                        y: angle.cos(),
                    })
                    .insert(Laser)
                    .insert(FromPlayer)
                    .insert(SpriteSize::from(PLAYER_LASER_SIZE));
            };

            const WEAPON_LVL: u32 = 5;
            const DISPERSION: f32 = PI / 3.;
            const ANGLE_0: f32 = -DISPERSION / 2.;
            const DELTA: f32 = DISPERSION / (WEAPON_LVL + 1) as f32;

            let mut angle = ANGLE_0 + DELTA;

            for _ in 0..WEAPON_LVL {
                spawn_laser(angle);
                angle += DELTA;
            }
        }
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.
        } else if kb.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        }
    }
}
