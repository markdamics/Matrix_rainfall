use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use anyhow::{Context, Result};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{cursor, queue, terminal};
use rand::Rng;

const CHARS: [&str; 2] = ["1", "0"];

#[derive(Clone)]
struct Column {
    length: u16,
    charachter_color: Color,
}

impl Column {
    fn new(length: u16, charachter_color: Color) -> Self {
        Self {
            length,
            charachter_color
        }
    }

    fn init(&self, stdout: &mut std::io::Stdout) -> Result<()> {
        for _ in 0..self.length {
            let random_char = get_random_char();
            queue!(stdout, Print(random_char.to_string()))?;
        }
        Ok(())
    }

    fn update(&self) -> Result<()> {
        Ok(())
    }
}

struct Matrix {
    width: u16,
    height: u16,
    charachter_color: Color,
    background_color: Color,
    columns: Vec<Column>,
}

impl Matrix {
    fn new(width: u16, height: u16, charachter_color: Color, background_color: Color) -> Self {
        Self {
            width,
            height,
            charachter_color,
            background_color,
            columns: vec![Column::new(height, charachter_color); width as usize]
        }
    }

    fn init(&self, stdout: &mut std::io::Stdout) -> Result<()> {
        queue!(stdout, SetBackgroundColor(self.background_color))?;
        queue!(stdout, SetForegroundColor(self.charachter_color))?;

        for col in &self.columns {
            col.init(stdout)?;
        }

        queue!(stdout, ResetColor)?;
        queue!(stdout, cursor::MoveTo(0, 0))?;

        Ok(())
    }

    fn update(&self) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {

    let mut stdout = stdout();
    let (width, height) = terminal::size().context("Get terminal size")?;
    let charachter_color = Color::Rgb { r: 0, g: 255, b: 0 };
    let background_color = Color::Rgb { r: 0, g: 0, b: 0 };

    queue!(stdout, cursor::MoveTo(0, 0))?;
    queue!(stdout, SetBackgroundColor(background_color))?;
    queue!(stdout, SetForegroundColor(charachter_color))?;

    let matrix = Matrix::new(width, height, charachter_color, background_color);

    loop{
        matrix.init(&mut stdout).context("Initialize matrix")?;

        stdout.flush()?;
        sleep(Duration::from_millis(1000));
    }
    Ok(())
}

fn get_random_char() -> String {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..CHARS.len());
    CHARS[random_index].to_string()
}
