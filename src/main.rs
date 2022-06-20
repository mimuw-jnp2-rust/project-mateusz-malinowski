#![allow(clippy::type_complexity)]

mod collision;
mod components;
mod enemy;
mod explosion;
mod load;
mod movement;
mod player;
mod save;
mod ui;

use crate::collision::CollisionPlugin;
use crate::components::{
    Enemy, FromEnemy, FromPlayer, Laser, Movable, Player, SpriteSize, Velocity,
};
use crate::enemy::EnemyPlugin;
use crate::explosion::ExplosionPlugin;
use crate::load::LoadPlugin;
use crate::movement::MovementPlugin;
use crate::player::PlayerPlugin;
use crate::save::SavePlugin;
use crate::ui::main_menu::MainMenuPlugin;
use crate::ui::pause_menu::PauseMenuPlugin;
use crate::ui::score::ScorePlugin;
use bevy::prelude::*;
use enemy::{ENEMY_LASER_SPRITE, ENEMY_SPRITE};
use explosion::EXPLOSION_SHEET;
use player::{PLAYER_LASER_SPRITE, PLAYER_SPRITE};

const SPRITE_SCALE: f32 = 0.5;

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
    Save,
    Load,
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
        .add_plugin(SavePlugin)
        .add_plugin(LoadPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(ExplosionPlugin)
        .add_startup_system(setup_system)
        .add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(pause_keyboard_event_system),
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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

    // create explosion texture atlas
    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4);
    let explosion = texture_atlases.add(texture_atlas);

    // Game Textures
    commands.insert_resource(GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion,
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

// Despawns game entities when entering to main menu.
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

    app_state.set(AppState::InGame).unwrap();
}
