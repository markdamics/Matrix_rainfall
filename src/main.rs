mod matrix;
mod column;
mod symbol;

use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;
use anyhow::{Context, Result};
use colors_transform::{Color, Rgb};
use crossterm::terminal;
use crate::matrix::Matrix;

fn main() -> Result<()> {

    let mut stdout = stdout();
    let (width, height) = terminal::size().context("Get terminal size")?;
    let character_color = "#04ff00";
    let background_color = "#000000";

    let mut matrix = Matrix::new(width, height, character_color, background_color);

    let frame_speed = (1000f64 / 10 as f64).round() as u64;

    loop{
        matrix.init(&mut stdout).context("Initialize matrix")?;
        matrix.update();

        sleep(Duration::from_millis(frame_speed));
    }
}

pub fn get_crossterm_color(color: &str) -> crossterm::style::Color {
    let rgb_color = Rgb::from_hex_str(color).unwrap();
    let red = rgb_color.get_red();
    let green = rgb_color.get_green();
    let blue = rgb_color.get_blue();
    crossterm::style::Color::Rgb { r: red as u8, g: green as u8, b: blue as u8 }
}
