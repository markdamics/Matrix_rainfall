mod wall;
mod columns;

use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use crossterm::{execute, style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}, ExecutableCommand, event, terminal, queue};
use anyhow::{Context, Result};
use rand::Rng;
use crate::wall::Wall;

fn main() -> Result<()> {

 let mut stdout = stdout();
 let base_color = Color::Rgb { r: 0, g: 255, b: 0 };

 stdout.execute(terminal::Clear(terminal::ClearType::All))?;

 let (width, height) = terminal::size().context("Get terminal size")?;

 let mut wall = Wall::new(width as u32, height as u32);\
 let mut rand = rand::thread_rng().random_bool(0.1);

 loop{
        wall.render(&mut stdout);
        wall.step(rand);
        stdout.flush()?;
        sleep(Duration::from_millis(100));
 }
}
