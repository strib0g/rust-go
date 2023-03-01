mod board;
use crate::board::{Board, Stone};
use bevy::{
    prelude::*, 
    window::{WindowDescriptor, PresentMode}, 
    DefaultPlugins, 
    diagnostic::LogDiagnosticsPlugin};
use bevy_prototype_lyon::{prelude::{ShapePlugin, GeometryBuilder, DrawMode, FillMode}, shapes, entity::ShapeBundle};
fn main() {
    App::new()
    .insert_resource(ClearColor(Color::hex("A88332").unwrap()))
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
    .add_plugin(ShapePlugin)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_startup_system(setup)
    .add_startup_system(init_board)
    .run();


}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

fn init_board(mut commands: Commands) {
    let board = Board::new();
    commands.spawn(board);

    let shape = shapes::Rectangle{
        extents: Vec2{x:10.0, y:10.0,},
        origin: shapes::RectangleOrigin::TopLeft,
    };

   // commands.spawn(GeometryBuilder::build_as(&shape, DrawMode::Fill(FillMode::color(Color::ALICE_BLUE)), Transform::default()));

    for i in 0..board.get_size().0 {
        for j in 0..board.get_size().1 {
            let pos = Transform::from_xyz((i as f32)*10.0, (j as f32)*10.0, 0.0);
            commands.spawn(GeometryBuilder::build_as(&shape, DrawMode::Fill(FillMode::color(Color::ALICE_BLUE)), pos));
        }
    }
}