use bevy::prelude::{Component};
use log::{debug, trace};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Stone {
    Black,
    White,
    None,
}

// Do we need to split the coordinates up?
// I guess it helps if for some reason I implement rectangular boards..
#[derive(Component, Clone, Copy)]
pub struct Field {
    coords: (f32, f32),
    stone: Stone,
}

impl Default for Field {
    fn default() -> Field {
        Field{coords: (0.0, 0.0), stone: Stone::None}
    }
}

#[derive(Component, Clone, Copy)]
pub struct Board {
    grid: [[Field; 19]; 19],
}

impl Board {
    pub fn new () -> Self{
        let new_row : [Field; 19] = [Field::default(); 19];
        let new_grid : [[Field; 19]; 19] = [new_row; 19];
        
        Self { grid : new_grid}

    }

    pub fn place_stone(&mut self, x:usize, y:usize, stone: Stone) -> bool {
        if self.grid[x][y].stone != Stone::None {
            debug!("Could not place stone in position {x}, {y}");
            return false;
        }

        self.grid[x][y].stone = stone;
        debug!("Stone placed in position {x}, {y}");
        return true;
    }

    pub fn get_size(&self) -> (u32, u32) {
        let x = self.grid[0].len() as u32;
        let y = self.grid.len() as u32;
        (x, y)
    }

    pub fn get_field(&mut self, x: u32, y: u32) -> &mut Field {
        &mut self.grid[x as usize][y as usize]
    }
}

impl Field {
    pub fn set_coords(&mut self, new_coords: (f32, f32)) {
        trace!("Setting field coordinates: {}, {}", new_coords.0, new_coords.1);
        self.coords = new_coords
    }
    pub fn get_coords(&self) -> (f32, f32){
        (self.coords.0, self.coords.1)
    }
}