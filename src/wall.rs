use crossterm::queue;
use crossterm::style::{SetBackgroundColor, SetForegroundColor};

use crate::columns::Columns;

#[derive(Clone)]
pub struct Wall {
    pub width: u32,
    pub height: u32,
    pub columns: Vec<Columns>
}

impl Wall {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            columns: vec![Columns::new(height); width as usize]
        }
    }

    pub fn render(&self, stdout: &mut std::io::Stdout) {

    }

    pub fn step(&mut self, rand: bool) {
        for column in &mut self.columns {
            column.step();
        }
    }
}