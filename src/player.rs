use crate::components::{Movable, Player, SpriteSize, Velocity};
use crate::weapons::WeaponType;
use crate::{AppState, GameTextures, WindowSize, SPRITE_SCALE};
use bevy::prelude::*;

pub const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);

pub struct PlayerState {
    pub lives: u32,
    pub weapon_type: WeaponType,
    pub weapon_lvl: u32,
}

impl PlayerState {
    pub fn new() -> PlayerState {
        PlayerState {
            lives: 3,
            weapon_type: WeaponType::Lasergun,
            weapon_lvl: 1,
        }
    }

    pub fn reset(&mut self) {
        self.lives = 3;
        self.weapon_type = WeaponType::Lasergun;
        self.weapon_lvl = 1;
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    // .with_system(player_fire_system2)
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
