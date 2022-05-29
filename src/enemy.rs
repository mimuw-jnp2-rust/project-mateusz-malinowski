use crate::components::Enemy;
use crate::{
    EnemyCount, GameTextures, Movable, Rotated, SpriteSize, Velocity, Wave, WindowSize, ENEMY_SIZE,
    ENEMY_SPRITE, SPRITE_SCALE, TIME_STEP,
};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_spawn_system)
            .add_system(enemy_movement_system);
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
