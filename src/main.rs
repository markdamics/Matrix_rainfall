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
    vector_start_index: usize,
    vector_end_index: usize,
}

impl Column {
    fn new(length: u16, character_color: Color) -> Self {
        Self {
            length,
            character_color,
            characters: vec![' '; length as usize],
            vector_start_index: 0,
            vector_end_index: 0
        }
    }

    fn init(&self, stdout: &mut std::io::Stdout, height_index: usize) -> Result<()> {
        queue!(stdout, SetForegroundColor(self.character_color))?;
        //height_index is the column index
        queue!(stdout, Print(self.characters[height_index])).context("initialize with empty characters")?;
        Ok(())
    }

    fn update(&mut self) -> () {
        let random_index = rand::rng().random_range(0..100);
        let rand_distance: usize = rand::rng().random_range(self.length/3..self.length/2) as usize;

        if self.vector_start_index == 0 && random_index > 5 {
            if self.vector_end_index < self.characters.len() {
                self.characters[self.vector_end_index] = ' ';
                self.vector_end_index = (self.vector_end_index + 1) % self.characters.len();
            }
            return;
        }

        if self.vector_start_index < self.characters.len() {
            self.characters[self.vector_start_index] = get_random_char();
            self.vector_start_index = (self.vector_start_index + 1) % self.characters.len();
        }

        if self.vector_start_index >= rand_distance && self.vector_end_index < self.characters.len() {
            self.characters[self.vector_end_index] = ' ';
            self.vector_end_index = (self.vector_end_index + 1) % self.characters.len();
        }
    }
}

struct Matrix {
    height: u16,
    background_color: Color,
    columns: Vec<Column>,
}

impl Matrix {
    fn new(width: u16, height: u16, character_color: Color, background_color: Color) -> Self {
        Self {
            height,
            background_color,
            columns: vec![Column::new(height, character_color); width as usize]
        }
    }

    fn init(&self, stdout: &mut std::io::Stdout) -> Result<()> {
        queue!(stdout, cursor::Hide)?;
        queue!(stdout, cursor::MoveTo(0, 0))?;
        queue!(stdout, SetBackgroundColor(self.background_color))?;

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

    fn update(&mut self) -> () {
        for col in &mut self.columns {
            col.update();
        }
    }
}

fn main() -> Result<()> {

    let mut stdout = stdout();
    let (width, height) = terminal::size().context("Get terminal size")?;
    let character_color = Color::Rgb { r: 0, g: 255, b: 0 };
    let background_color = Color::Rgb { r: 0, g: 0, b: 0 };

    let mut matrix = Matrix::new(width, height, character_color, background_color);

    loop{
        matrix.init(&mut stdout).context("Initialize matrix")?;
        matrix.update();

        sleep(Duration::from_millis(50));
    }
}

fn get_random_char() -> char {
    let mut rng = rand::rng();
    let random_index = rng.random_range(0..CHARS.len());
    CHARS[random_index]
}
