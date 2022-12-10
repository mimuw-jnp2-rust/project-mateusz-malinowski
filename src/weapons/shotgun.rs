use crate::weapons::{fire_criteria, WeaponType};
use crate::{
    AppState, FromPlayer, GameTextures, Laser, Movable, Player, PlayerState, SpriteSize, Velocity,
    SPRITE_SCALE,
};

use bevy::prelude::*;
use std::f32::consts::PI;

pub const SHOTGUN_SPRITE: &str = "laserGreen12.png";
const SHOTGUN_SIZE: (f32, f32) = (13., 37.);

pub struct ShotgunPlugin;

impl Plugin for ShotgunPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_run_criteria(fire_criteria::<{ WeaponType::Shotgun }>)
                .with_system(fire_system),
        );
    }
}

fn fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>,
    player_state: Res<PlayerState>,
) {
    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);

            let mut spawn_bullet = |angle: f32| {
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: game_textures.shotgun_bullet.clone(),
                        transform: Transform {
                            translation: Vec3::new(x, y + 15., 5.),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                            rotation: Quat::from_rotation_z(-angle),
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
                    .insert(SpriteSize::from(SHOTGUN_SIZE));
            };

            let dispersion = PI / 3.;
            let angle_0 = -dispersion / 2.;
            let delta = dispersion / (player_state.weapon_lvl + 1) as f32;

            let mut angle = angle_0 + delta;

            for _ in 0..player_state.weapon_lvl {
                spawn_bullet(angle);
                angle += delta;
            }
        }
    }
}
