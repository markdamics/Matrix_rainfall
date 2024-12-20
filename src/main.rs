use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use anyhow::{Context, Result};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{cursor, queue, terminal};
use rand::Rng;

const CHARS: [char; 2] = ['1', '0'];

#[derive(Clone)]
struct Column {
    length: u16,
    character_color: Color,
    characters: Vec<char>,
    vector_index: usize
}

impl Column {
    fn new(length: u16, character_color: Color) -> Self {
        Self {
            length,
            character_color,
            characters: vec![' '; length as usize],
            vector_index: 0
        }
    }

    fn init(&self, stdout: &mut std::io::Stdout) -> Result<()> {
        for (i, char) in self.characters.iter().enumerate() {
            queue!(stdout, SetForegroundColor(self.character_color))?;
            queue!(stdout, Print(char)).context("initialize with empty characters")?;
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        self.characters[self.vector_index] = get_random_char();
        self.vector_index += 1;

        if self.vector_index >= self.length as usize {
            self.vector_index = 0;
        }
        //Why does it "moving" in rows?
        Ok(())
    }
}

struct Matrix {
    width: u16,
    height: u16,
    character_color: Color,
    background_color: Color,
    columns: Vec<Column>,
}

impl Matrix {
    fn new(width: u16, height: u16, character_color: Color, background_color: Color) -> Self {
        Self {
            width,
            height,
            character_color,
            background_color,
            columns: vec![Column::new(height, character_color); width as usize]
        }
    }

    fn init(&self, stdout: &mut std::io::Stdout) -> Result<()> {
        queue!(stdout, cursor::MoveTo(0, 0))?;
        queue!(stdout, SetBackgroundColor(self.background_color))?;

        for col in &self.columns {
            col.init(stdout)?;
        }

        queue!(stdout, ResetColor)?;
        queue!(stdout, cursor::Show)?;

        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        for col in &mut self.columns {
            col.update()?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {

    let mut stdout = stdout();
    let (width, height) = terminal::size().context("Get terminal size")?;
    let character_color = Color::Rgb { r: 0, g: 255, b: 0 };
    let background_color = Color::Rgb { r: 0, g: 0, b: 0 };

    queue!(stdout, cursor::MoveTo(0, 0))?;
    queue!(stdout, SetBackgroundColor(background_color))?;
    queue!(stdout, SetForegroundColor(character_color))?;

    let mut matrix = Matrix::new(width, height, character_color, background_color);

    loop{
        matrix.init(&mut stdout).context("Initialize matrix")?;
        matrix.update().context("Update matrix")?;

        stdout.flush()?;
        sleep(Duration::from_millis(500));
    }
    Ok(())
}

fn get_random_char() -> char {
    let mut rng = rand::rng();
    let random_index = rng.random_range(0..CHARS.len());
    CHARS[random_index]
}
