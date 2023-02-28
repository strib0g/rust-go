mod board;
use crate::board::{Board, Stone};
fn main() {
    env_logger::init();
    let mut game_board: Board = Board::new();

    game_board.place_stone(0, 0, Stone::Black);
    game_board.place_stone(1, 1, Stone::White);

    game_board.place_stone(0, 0, Stone::White);

}
