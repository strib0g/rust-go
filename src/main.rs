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
            width: 800.,
            height: 600.,
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
    debug!("Begin board initialization");
    let mut board = Board::new();
    commands.spawn(board);

    let shape = shapes::Rectangle{
        extents: Vec2{x:25.0, y:25.0,},
        origin: shapes::RectangleOrigin::TopLeft,
    };

    //commands.spawn(GeometryBuilder::build_as(&shape, DrawMode::Fill(FillMode::color(Color::ALICE_BLUE)), Transform::from_xyz(-250.0, -250.0, 0.0)));

    // TODO use constants here
    debug!("Begin spawning fields");
    for i in 0..board.get_size().0 {
        for j in 0..board.get_size().1 {
            let x_coordinate = ((i as f32) - 9.0)*26.3;
            let y_coordinate = ((j as f32)-9.0)*26.3;

            let field = board.get_field(i, j);
            field.set_coords((x_coordinate, y_coordinate));

            trace!("Spawning field at {x_coordinate}, {y_coordinate}");
            let pos = Transform::from_xyz(x_coordinate, y_coordinate, 0.0);     // Janky, but better than setting it and then getting it twice from field reference
            commands.spawn(GeometryBuilder::build_as(&shape, DrawMode::Fill(FillMode::color(Color::BISQUE)), pos));
        }
    }
}