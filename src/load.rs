use crate::{AppState, EnemyCount, PlayerState, Score, Wave};
use bevy::prelude::*;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Load).with_system(load_system));
    }
}

fn load_system(
    mut wave: ResMut<Wave>,
    mut enemy_count: ResMut<EnemyCount>,
    mut score: ResMut<Score>,
    mut player_state: ResMut<PlayerState>,
    mut app_state: ResMut<State<AppState>>,
) {
    let f = File::open("saves/save.txt");

    match f {
        Ok(file) => {
            let lines: Vec<io::Result<String>> = io::BufReader::new(file).lines().collect();
            wave.0 = lines[0].as_ref().unwrap().parse().unwrap();
            score.0 = lines[1].as_ref().unwrap().parse().unwrap();
            player_state.lives = lines[2].as_ref().unwrap().parse().unwrap();

            enemy_count.0 = 0;
        }
        Err(err) => {
            eprintln!("{}", err);

            // return to main menu
            app_state.set(AppState::MainMenu).unwrap();
            return;
        }
    }

    app_state.set(AppState::InGame).unwrap();
}
