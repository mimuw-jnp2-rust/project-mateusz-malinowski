use crate::weapons::lasergun::LasergunPlugin;
use crate::weapons::shotgun::ShotgunPlugin;
use crate::PlayerState;
use bevy::app::PluginGroupBuilder;
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;

pub mod lasergun;
pub mod shotgun;

#[derive(PartialEq, Eq)]
pub enum WeaponType {
    Lasergun,
    Shotgun,
}

pub struct WeaponPlugins;

impl PluginGroup for WeaponPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(LasergunPlugin).add(ShotgunPlugin);
    }
}

pub fn fire_criteria<const T: WeaponType>(
    kb: Res<Input<KeyCode>>,
    player_state: Res<PlayerState>,
) -> ShouldRun {
    if kb.just_pressed(KeyCode::Space) && player_state.weapon_type == T {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
