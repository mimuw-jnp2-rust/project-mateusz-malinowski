use bevy::prelude::*;
use rand::{Rng, thread_rng};
use crate::{ENEMY_SIZE, ENEMY_SPRITE, EnemyCount, GameTextures, SPRITE_SCALE, SpriteSize, Wave, WindowSize};
use crate::components::Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_spawn_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WindowSize>,
    mut enemy_count: ResMut<EnemyCount>,
    wave: Res<Wave>
) {
    // compute the random position
    const MARGIN: f32 = 100.;
    let mut rng = thread_rng();
    let w_span = win_size.w / 2. - MARGIN;
    let h_span = win_size.h / 2. - MARGIN;

    if enemy_count.0 == 0 {
        for _ in 0..wave.0 {
            let x = rng.gen_range(-w_span..w_span) as f32;
            let y = rng.gen_range(-h_span..h_span) as f32;

            commands
                .spawn_bundle( SpriteBundle {
                    texture: game_textures.enemy.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, 10.),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(Enemy)
                .insert(SpriteSize::from(ENEMY_SIZE));
        }

        enemy_count.0 = wave.0;
    }

}