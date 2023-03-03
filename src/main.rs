mod board;
use crate::board::{Board, Stone};
use bevy::{
    prelude::*, 
    window::{WindowDescriptor, PresentMode}, 
    DefaultPlugins, 
    diagnostic::LogDiagnosticsPlugin};
use bevy_prototype_lyon::{prelude::{ShapePlugin, GeometryBuilder, DrawMode, FillMode}, shapes, entity::ShapeBundle};

const RESOLUTION_X: f32 = 800.;
const RESOLUTION_Y: f32 = 600.;

const NO_TILES: f32 = 19.;    // Used to calculate space between tiles. . Should be set in game options in the future.

// TODO how do i margins
const TILE_COORDINATE_OFFSET: f32 = NO_TILES/2.-1.;
const TILE_SIZE_FACTOR: f32 = RESOLUTION_Y/NO_TILES;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::hex("A88332").unwrap()))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "go-rust".to_string(),
            width: RESOLUTION_X,
            height: RESOLUTION_Y,
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


    // TODO use constants here
    debug!("Begin spawning fields");
    for i in 0..board.get_size().0 {
        for j in 0..board.get_size().1 {
            let x_coordinate = ((i as f32) - TILE_COORDINATE_OFFSET)*TILE_SIZE_FACTOR;
            let y_coordinate = ((j as f32) - TILE_COORDINATE_OFFSET)*TILE_SIZE_FACTOR;

            let field = board.get_field(i, j);
            field.set_coords((x_coordinate, y_coordinate));

            trace!("Spawning field at {x_coordinate}, {y_coordinate}");
            let pos = Transform::from_xyz(x_coordinate, y_coordinate, 0.0);     // Janky, but better than setting it and then getting it twice from field reference
            commands.spawn(GeometryBuilder::build_as(&shape, DrawMode::Fill(FillMode::color(Color::GRAY)), pos));
        }
    }
}