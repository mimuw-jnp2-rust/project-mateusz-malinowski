#![allow(unused)] // todo: to remove

mod player;

use std::mem;
// use std::default::Default;
use bevy::prelude::*;
use crate::CursorIcon::Default;
use crate::player::PlayerPlugin;
// use crate::CursorIcon::Default;

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

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "shooter game".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>
) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Window Size
    let window = windows.get_primary_mut().unwrap();

    commands.insert_resource(
        WindowSize {
            w: window.width(),
            h: window.height()
        }
    );

    // Game Textures
    commands.insert_resource(
        GameTextures {
            player: asset_server.load(PLAYER_SPRITE),
            player_laser: asset_server.load(PLAYER_LASER_SPRITE),
            enemy: asset_server.load(ENEMY_SPRITE),
            enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
            explosion: asset_server.load(EXPLOSION_SHEET),
        }
    )
}
