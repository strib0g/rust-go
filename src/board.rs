use log::debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Stone {
    Black,
    White,
    None,
}

impl Default for Stone {
    fn default() -> Self {
        Stone::None
    }
}

pub struct Board {
    grid: [[Stone; 19]; 19],
}

impl Board {
    pub fn new () -> Self{
        let new_row : [Stone; 19] = [Stone::None; 19];
        let new_grid : [[Stone; 19]; 19] = [new_row; 19];
        Self { grid : new_grid}
    }

    pub fn place_stone(&mut self, x:usize, y:usize, stone: Stone) -> bool {
        if self.grid[x][y] != Stone::None {
            debug!("Could not place stone in position {x}, {y}");
            return false;
        }

        self.grid[x][y] = stone;
        debug!("stone placed in position {x}, {y}");
        return true;
    }
}