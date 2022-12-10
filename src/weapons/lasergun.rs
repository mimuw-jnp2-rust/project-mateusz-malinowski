use crate::weapons::{fire_criteria, WeaponType};
use crate::{
    AppState, FromPlayer, GameTextures, Laser, Movable, Player, PlayerState, SpriteSize, Velocity,
    SPRITE_SCALE,
};

use bevy::prelude::*;

pub const LASERGUN_SPRITE: &str = "laser_a_01.png";
const LASGUNER_SIZE: (f32, f32) = (9., 54.);

pub struct LasergunPlugin;

impl Plugin for LasergunPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_run_criteria(fire_criteria::<{ WeaponType::Lasergun }>)
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

            let mut spawn_bullet = |x_offset: f32, y_offset: f32| {
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: game_textures.lasergun_bullet.clone(),
                        transform: Transform {
                            translation: Vec3::new(x + x_offset, y + y_offset, 5.),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Movable { auto_despawn: true })
                    .insert(Velocity { x: 0., y: 1. })
                    .insert(Laser)
                    .insert(FromPlayer)
                    .insert(SpriteSize::from(LASGUNER_SIZE));
            };

            let x_offset_step = 8.;
            let y_offset = 30.;
            let y_offset_step = 3.;

            if player_state.weapon_lvl % 2 == 1 {
                spawn_bullet(0., y_offset)
            }

            for i in 1..=player_state.weapon_lvl / 2 {
                spawn_bullet(
                    x_offset_step * i as f32,
                    y_offset - y_offset_step * i as f32,
                );
                spawn_bullet(
                    -x_offset_step * i as f32,
                    y_offset - y_offset_step * i as f32,
                );
            }
        }
    }
}
