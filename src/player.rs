use bevy::prelude::*;
use crate::{BASE_SPEED, GameTextures, PLAYER_SIZE, SPRITE_SCALE, TIME_STEP, WindowSize};
use crate::components::{Player, Velocity};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_movement_system)
            .add_system(player_keyboard_event_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut window_size: ResMut<WindowSize>
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
        .insert(Velocity {x: 0., y: 0.});
}

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        transform.translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
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