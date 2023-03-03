//TODO: The amount of casting in this file can't be right

use bevy::prelude::{Component};
use log::{debug, trace};

#[derive(Component, Clone, Copy)]
pub struct Field {
    coords: (f32, f32),
    stone: crate::StoneColor,   // TODO change this to be an option
}

impl Default for Field {
    fn default() -> Field {
        Field{coords: (0.0, 0.0), stone: crate::StoneColor::None}
    }
}

#[derive(Component, Clone, Copy)]
pub struct Board {
    grid: [[Field; 19]; 19],
}

impl Board {
    pub fn new () -> Self{
        let new_row : [Field; crate::NO_TILES as usize] = [Field::default(); crate::NO_TILES as usize];
        let new_grid : [[Field; crate::NO_TILES as usize]; crate::NO_TILES as usize] = [new_row; crate::NO_TILES as usize];
        
        Self { grid : new_grid}

    }

    pub fn place_stone(&mut self, x:usize, y:usize, stone: crate::StoneColor) -> bool {
        if self.grid[x][y].stone != crate::StoneColor::None {
            debug!("Could not place stone in position {x}, {y}");
            return false;
        }

        self.grid[x][y].stone = stone;
        debug!("crate::StoneColor placed in position {x}, {y}");
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