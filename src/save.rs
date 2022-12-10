use crate::{AppState, PlayerState, Score, Wave};
use bevy::prelude::*;
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
    wave: ResMut<Wave>,
    score: ResMut<Score>,
    player_state: ResMut<PlayerState>,
    mut app_state: ResMut<State<AppState>>,
) {
    let text = format!("{}\n{}\n{}\n", wave.0, score.0, player_state.lives);

    if !Path::new("saves").exists() {
        match fs::create_dir("saves") {
            Ok(_) => {}
            Err(err) => {
                eprintln!("{}", err);

                // return to pause menu
                app_state.set(AppState::Paused).unwrap();
                return;
            }
        }
    }

    let f = File::create("saves/save.txt");

    match f {
        Ok(mut file) => {
            file.write_all(text.as_ref()).unwrap();
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }

    app_state.set(AppState::Paused).unwrap();
}
