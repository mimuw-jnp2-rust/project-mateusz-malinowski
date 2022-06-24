use crate::components::{Explosion, ExplosionTimer};
use crate::weapons::WeaponType;
use crate::{
    AppState, Enemy, EnemyCount, FromEnemy, FromPlayer, GameTextures, Laser, Player, PlayerState,
    Score, SpriteSize, Wave,
};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use std::collections::HashSet;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(player_laser_hit_enemy_system)
                .with_system(enemy_laser_hit_player_system),
        );
    }
}

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
    mut enemy_count: ResMut<EnemyCount>,
    mut wave: ResMut<Wave>,
    mut score: ResMut<Score>,
    game_textures: Res<GameTextures>,
    mut player_state: ResMut<PlayerState>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    // iterate through the lasers
    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        if despawned_entities.contains(&laser_entity) {
            continue;
        }

        let laser_scale = laser_tf.scale.xy();

        // iterate through the enemies
        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            if despawned_entities.contains(&enemy_entity)
                || despawned_entities.contains(&laser_entity)
            {
                continue;
            }

            let enemy_scale = enemy_tf.scale.xy();

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
                    player_state.weapon_lvl += 1;
                    match player_state.weapon_type {
                        WeaponType::Lasergun => player_state.weapon_type = WeaponType::Shotgun,
                        WeaponType::Shotgun => player_state.weapon_type = WeaponType::Lasergun,
                    }
                }

                // remove the laser
                commands.entity(laser_entity).despawn();
                despawned_entities.insert(laser_entity);

                // spawn the Explosion
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: game_textures.explosion.clone(),
                        transform: Transform {
                            translation: enemy_tf.translation,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Explosion)
                    .insert(ExplosionTimer::default());
            }
        }
    }
}

fn enemy_laser_hit_player_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromEnemy>)>,
    player_query: Query<(&Transform, &SpriteSize), With<Player>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if let Ok((player_tf, player_size)) = player_query.get_single() {
        let player_scale = player_tf.scale.xy();

        for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
            let laser_scale = laser_tf.scale.xy();

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

                player_state.lives -= 1;

                if player_state.lives == 0 {
                    // return to main menu
                    app_state.set(AppState::MainMenu).unwrap();
                }

                break;
            }
        }
    }
}
