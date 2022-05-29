use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
}

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct FromEnemy;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct ExitButton;

#[derive(Component)]
pub struct NewGameButton;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub struct ContinueButton;

#[derive(Component)]
pub struct MainMenuButton;
