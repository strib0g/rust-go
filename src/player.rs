use bevy::prelude::{Component};
use log::{debug, trace};

#[derive(Component, Clone, Copy)]
pub struct Player {
    score: u32,
    color: crate::StoneColor
}

impl Player {
    pub fn new(new_color:crate::StoneColor) -> Self {
        Self{score:0, color:new_color}
    }
}

