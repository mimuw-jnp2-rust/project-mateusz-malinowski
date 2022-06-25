use crate::components::{PowerUp, SpawnPowerUp};
use crate::{AppState, Movable, SpriteSize, Velocity};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PowerUpType {
    Hp,
    WeaponLvl,
    ChangeToLaser,
    ChangeToShotgun,
}

const POWER_UP_TYPES_CNT: usize = 4;

struct PowerUpTextures {
    hp: Handle<Image>,
    weapon_lvl: Handle<Image>,
    change_to_laser: Handle<Image>,
    change_to_shotgun: Handle<Image>,
}

const HP_SPRITE: &str = "hp.png";
const WEAPON_LVL_SPRITE: &str = "power_up.png";
const CHANGE_TO_LASER_SPRITE: &str = "blue_weapon.png";
const CHANGE_TO_SHOTGUN_SPRITE: &str = "green_weapon.png";

const POWER_UP_SIZE: (f32, f32) = (34., 34.);

pub struct PowerUpsPlugin;

impl Plugin for PowerUpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(power_ups_setup_system)
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(power_up_spawn_system),
            );
    }
}

pub fn get_random_power_up() -> PowerUpType {
    let n = thread_rng().gen_range(0..POWER_UP_TYPES_CNT);
    match n {
        0 => PowerUpType::Hp,
        1 => PowerUpType::WeaponLvl,
        2 => PowerUpType::ChangeToLaser,
        _ => PowerUpType::ChangeToShotgun,
    }
}

fn power_ups_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PowerUpTextures {
        hp: asset_server.load(HP_SPRITE),
        weapon_lvl: asset_server.load(WEAPON_LVL_SPRITE),
        change_to_laser: asset_server.load(CHANGE_TO_LASER_SPRITE),
        change_to_shotgun: asset_server.load(CHANGE_TO_SHOTGUN_SPRITE),
    });
}

fn power_up_spawn_system(
    mut commands: Commands,
    textures: Res<PowerUpTextures>,
    power_ups_to_spawn: Query<(Entity, &SpawnPowerUp)>,
) {
    for (entity, power_up_to_spawn) in power_ups_to_spawn.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: match power_up_to_spawn.type_ {
                    PowerUpType::Hp => textures.hp.clone(),
                    PowerUpType::WeaponLvl => textures.weapon_lvl.clone(),
                    PowerUpType::ChangeToLaser => textures.change_to_laser.clone(),
                    PowerUpType::ChangeToShotgun => textures.change_to_shotgun.clone(),
                },
                transform: Transform {
                    translation: power_up_to_spawn.translation,
                    // scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Movable { auto_despawn: true })
            .insert(Velocity { x: 0., y: -0.3 })
            .insert(PowerUp(power_up_to_spawn.type_))
            .insert(SpriteSize::from(POWER_UP_SIZE));

        // despawn the SpawnPowerUp entity
        commands.entity(entity).despawn();
    }
}
