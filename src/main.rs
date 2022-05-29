#![allow(unused)] // todo: to remove

mod components;
mod enemy;
mod player;
mod ui;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use std::collections::HashSet;

use crate::components::{
    Enemy, FromEnemy, FromPlayer, Laser, Movable, Player, ScoreText, SpriteSize, Velocity,
};
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;
use crate::ui::main_menu::MainMenuPlugin;
use crate::ui::pause_menu::PauseMenuPlugin;
use crate::ui::score::ScorePlugin;

// Asset Constants
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_SIZE: (f32, f32) = (144., 75.);
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const ENEMY_LASER_SIZE: (f32, f32) = (17., 55.);

const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const EXPLOSION_LEN: usize = 16;

const SPRITE_SCALE: f32 = 0.5;

// Game Constants
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

// Font Constants
const SCORE_FONT: &str = "fonts/LED Dot-Matrix.ttf";
const BUTTON_FONT: &str = "fonts/Instruction.otf";

// Resources
pub struct WindowSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>,
}

pub struct Fonts {
    score: Handle<Font>,
    button: Handle<Font>,
}

struct Wave(u32);

struct EnemyCount(u32);

struct Rotated(bool);

struct Score(u32);

struct Lives(u32);

// States
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InitNewGame,
    InGame,
    Paused,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "shooter game".to_string(),
            ..default()
        })
        .add_state(AppState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(PauseMenuPlugin)
        .add_startup_system(setup_system)
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(movable_system)
                .with_system(player_laser_hit_enemy_system)
                .with_system(enemy_laser_hit_player_system)
                .with_system(pause_keyboard_event_system),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Paused).with_system(continue_keyboard_event_system),
        )
        .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(despawn_system))
        .add_system_set(
            SystemSet::on_enter(AppState::InitNewGame).with_system(new_game_init_system),
        )
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Window Size
    let window = windows.get_primary_mut().unwrap();

    commands.insert_resource(WindowSize {
        w: window.width(),
        h: window.height(),
    });

    // Game Textures
    commands.insert_resource(GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion: asset_server.load(EXPLOSION_SHEET),
    });

    // Fonts
    commands.insert_resource(Fonts {
        score: asset_server.load(SCORE_FONT),
        button: asset_server.load(BUTTON_FONT),
    });

    commands.insert_resource(Wave(1));
    commands.insert_resource(EnemyCount(0));
    commands.insert_resource(Rotated(false));
    commands.insert_resource(Score(0));
    commands.insert_resource(Lives(3));
}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WindowSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        // despawn when out of screen
        if movable.auto_despawn {
            const MARGIN: f32 = 200.;

            if translation.y > win_size.h / 2. + MARGIN
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.w / 2. + MARGIN
                || translation.x < -win_size.w / 2. - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
    mut enemy_count: ResMut<EnemyCount>,
    mut wave: ResMut<Wave>,
    mut score: ResMut<Score>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    // iterate through the lasers
    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        if despawned_entities.contains(&laser_entity) {
            continue;
        }

        let laser_scale = Vec2::from(laser_tf.scale.xy());

        // iterate through the enemies
        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            if despawned_entities.contains(&enemy_entity)
                || despawned_entities.contains(&laser_entity)
            {
                continue;
            }

            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            // determine if collision
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            // perform collision
            if collision.is_some() {
                // remove the enemy
                commands.entity(enemy_entity).despawn();
                despawned_entities.insert(enemy_entity);
                enemy_count.0 -= 1;

                score.0 += 100;

                // start next wave
                if enemy_count.0 == 0 {
                    wave.0 += 1;
                }

                // remove the laser
                commands.entity(laser_entity).despawn();
                despawned_entities.insert(laser_entity);
            }
        }
    }
}

fn enemy_laser_hit_player_system(
    mut commands: Commands,
    mut lives: ResMut<Lives>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromEnemy>)>,
    player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if let Ok((player_entity, player_tf, player_size)) = player_query.get_single() {
        let player_scale = Vec2::from(player_tf.scale.xy());

        for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
            let laser_scale = Vec2::from(laser_tf.scale.xy());

            // determine if collision
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                player_tf.translation,
                player_size.0 * player_scale,
            );

            // perform the collision
            if collision.is_some() {
                // remove the laser
                commands.entity(laser_entity).despawn();

                lives.0 -= 1;

                if lives.0 == 0 {
                    // return to main menu
                    app_state.set(AppState::MainMenu);
                }

                break;
            }
        }
    }
}

fn pause_keyboard_event_system(kb: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if kb.pressed(KeyCode::Escape) || kb.pressed(KeyCode::P) {
        app_state.set(AppState::Paused).unwrap();
    }
}

fn continue_keyboard_event_system(kb: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if kb.pressed(KeyCode::Space) {
        app_state.set(AppState::InGame).unwrap();
    }
}

fn despawn_system(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    laser_query: Query<Entity, With<Laser>>,
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
    for laser_entity in laser_query.iter() {
        commands.entity(laser_entity).despawn();
    }
}

fn new_game_init_system(
    mut wave: ResMut<Wave>,
    mut enemy_count: ResMut<EnemyCount>,
    mut score: ResMut<Score>,
    mut lives: ResMut<Lives>,
    mut app_state: ResMut<State<AppState>>,
) {
    wave.0 = 1;
    enemy_count.0 = 0;
    score.0 = 0;
    lives.0 = 3;

    app_state.set(AppState::InGame);
}
