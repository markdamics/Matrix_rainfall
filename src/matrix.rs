use std::io::Write;
use crossterm::{cursor, queue};
use crossterm::style::{ResetColor, SetBackgroundColor};
use crate::column::Column;
use crate::get_crossterm_color;

pub struct Matrix {
    height: u16,
    background_color: String,
    columns: Vec<Column>,
}

impl Matrix {
    pub fn new(width: u16, height: u16, character_color: &str, background_color: &str) -> Self {
        Self {
            height,
            background_color: String::from(background_color),
            columns: vec![Column::new(width, height, character_color); width as usize]
        }
    }

    pub fn init(&self, stdout: &mut std::io::Stdout) -> anyhow::Result<()> {
        queue!(stdout, cursor::Hide)?;
        queue!(stdout, cursor::MoveTo(0, 0))?;

        let background_color = get_crossterm_color(&self.background_color);
        queue!(stdout, SetBackgroundColor(background_color))?;

        for i in 0..self.height { //Without it will "move" in rows
            for col in &self.columns {
                col.init(stdout, i as usize)?;
            }
        }

        queue!(stdout, ResetColor)?;
        queue!(stdout, cursor::Show)?;

        stdout.flush()?;

        Ok(())
    }

    pub fn update(&mut self) -> () {
        for col in &mut self.columns {
            col.update();
        }
    }
}