mod board;
use crate::board::{Board, Stone};
use bevy::{
    prelude::*, 
    window::{WindowDescriptor, PresentMode}, 
    DefaultPlugins, 
    diagnostic::LogDiagnosticsPlugin};
fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "go-rust".to_string(),
            width: 500.,
            height: 300.,
            present_mode: PresentMode::AutoVsync,
            ..default()
        },
        ..default()
    }))
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_startup_system(init_board)
    .run();


}

fn init_board(mut commands: Commands) {
    commands.spawn(Board::new());
}