use crate::components::Enemy;
use crate::{
    AppState, EnemyCount, FromEnemy, GameTextures, Laser, Movable, Rotated, SpriteSize, Velocity,
    Wave, WindowSize, SPRITE_SCALE,
};
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

// Asset Constants
pub const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_SIZE: (f32, f32) = (144., 75.);
pub const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const ENEMY_LASER_SIZE: (f32, f32) = (17., 55.);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(enemy_spawn_system)
                .with_system(enemy_movement_system),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_run_criteria(enemy_fire_criteria)
                .with_system(enemy_fire_system),
        );
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WindowSize>,
    mut enemy_count: ResMut<EnemyCount>,
    wave: Res<Wave>,
) {
    // compute the random position
    const MARGIN: f32 = 200.;
    const MARGIN_BOTTOM: f32 = 80.;

    let mut rng = thread_rng();
    let w_span = win_size.w / 2. - MARGIN;
    let h_span = win_size.h / 2. - MARGIN;

    if enemy_count.0 == 0 {
        for _ in 0..wave.0 {
            let x = rng.gen_range(-w_span..w_span) as f32;
            let y = rng.gen_range((-h_span + MARGIN_BOTTOM)..h_span) as f32;

            commands
                .spawn_bundle(SpriteBundle {
                    texture: game_textures.enemy.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, 10.),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(Enemy)
                .insert(SpriteSize::from(ENEMY_SIZE))
                .insert(Movable {
                    auto_despawn: false,
                })
                .insert(Velocity {
                    x: rng.gen_range(-0.3..0.3) as f32,
                    y: rng.gen_range(-0.3..0.3) as f32,
                });
        }

        enemy_count.0 = wave.0;
    }
}

fn enemy_movement_system(
    time: Res<Time>,
    mut rotated: ResMut<Rotated>,
    mut query: Query<&mut Velocity, With<Enemy>>,
) {
    let now = (time.seconds_since_startup() as f32 * 2.) as i32;

    if now % 2 == 0 {
        if !rotated.0 {
            if now % 4 != 0 {
                for mut velocity in query.iter_mut() {
                    velocity.x *= -1.;
                }
            } else {
                for mut velocity in query.iter_mut() {
                    velocity.y *= -1.;
                }
            }
            rotated.0 = true;
        }
    } else {
        rotated.0 = false;
    }
}

fn enemy_fire_criteria() -> ShouldRun {
    if thread_rng().gen_bool(1. / 60.) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    let count = enemy_query.iter().count();

    if count > 0 {
        let nth = thread_rng().gen_range(0..count);
        let &tf = enemy_query.iter().nth(nth).unwrap();
        let (x, y) = (tf.translation.x, tf.translation.y);

        // spawn enemy laser sprite
        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.enemy_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y - 15., 0.),
                    rotation: Quat::from_rotation_x(PI),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(SpriteSize::from(ENEMY_LASER_SIZE))
            .insert(FromEnemy)
            .insert(Movable { auto_despawn: true })
            .insert(Velocity { x: 0., y: -1. });
    }
}
