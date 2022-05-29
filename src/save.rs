use crate::{AppState, EnemyCount, Lives, Score, Wave};
use bevy::prelude::*;
use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Save).with_system(save_system));
    }
}

fn save_system(
    mut wave: ResMut<Wave>,
    mut enemy_count: ResMut<EnemyCount>,
    mut score: ResMut<Score>,
    mut lives: ResMut<Lives>,
    mut app_state: ResMut<State<AppState>>,
) {
    let text = format!("{}\n{}\n{}\n{}\n", wave.0, enemy_count.0, score.0, lives.0);

    if !Path::new("saves").exists() {
        match fs::create_dir("saves") {
            Ok(_) => {}
            Err(err) => {
                eprintln!("{}", err);

                // return to pause menu
                app_state.set(AppState::Paused);
                return;
            }
        }
    }

    let mut f = File::create("saves/save.txt");

    match f {
        Ok(mut file) => {
            file.write_all(text.as_ref()).unwrap();
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }

    app_state.set(AppState::Paused);
}
