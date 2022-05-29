use crate::{AppState, EnemyCount, Lives, Score, Wave};
use bevy::prelude::*;
use std::fmt::format;
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::Path;
use std::{fs, io};

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
    mut lives: ResMut<Lives>,
    mut app_state: ResMut<State<AppState>>,
) {
    let mut f = File::open("saves/save.txt");

    match f {
        Ok(mut file) => {
            let lines: Vec<io::Result<String>> = io::BufReader::new(file).lines().collect();
            wave.0 = lines[0].as_ref().unwrap().parse().unwrap();
            score.0 = lines[1].as_ref().unwrap().parse().unwrap();
            lives.0 = lines[2].as_ref().unwrap().parse().unwrap();

            enemy_count.0 = 0;
        }
        Err(err) => {
            eprintln!("{}", err);

            // return to main menu
            app_state.set(AppState::MainMenu).unwrap();
        }
    }

    app_state.set(AppState::InGame).unwrap();
}
