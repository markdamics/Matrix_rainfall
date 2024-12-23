use anyhow::{Context, Result};
use colors_transform::{Color, Hsl, Rgb};
use crossterm::queue;
use crossterm::style::{ Print, SetForegroundColor};
use rand::Rng;
use crate::get_crossterm_color;

const CHARS: [char; 2] = ['1', '0'];

#[derive(Clone)]
pub struct Symbol {
    character: char,
    color: String,
    is_first: bool
}

impl Symbol {
    pub fn new(character: char, color: &str) -> Self {
        Self {
            character,
            color: String::from(color),
            is_first: false
        }
    }

    // pub fn set_white_symbol() -> Self {
    //     Self {
    //         character: get_random_char(),
    //         color: String::from("#FFFFFF"),
    //     }
    // }

    pub fn set_first_symbol(color: &str) -> Self {
        Self {
            character: get_random_char(),
            color: String::from(color),
            is_first: true
        }
    }

    pub fn set_random_symbols(&mut self) -> () {
            self.character = get_random_char()
    }

    pub fn is_empty(&self) -> bool {
        self.character == ' '
    }

    pub fn set_empty() -> Self {
        Self {
            character: ' ',
            color: String::from("#000000"),
            is_first: false
        }
    }

    pub fn init(&self, stdout: &mut std::io::Stdout) -> Result<()> {
        let foreground_color = get_crossterm_color(&self.color);
        queue!(stdout, SetForegroundColor(foreground_color))?;
        queue!(stdout, Print(self.character.to_string())).context("initialize with empty characters")?;
        Ok(())
    }

    pub fn fade(&mut self) -> () {
        let rgb_color = Rgb::from_hex_str(&self.color).unwrap();
        let hex_color = rgb_color.to_hsl();
        let hue = hex_color.get_hue();
        let saturation = hex_color.get_saturation();
        let lightness = hex_color.get_lightness();
        let new_color = Hsl::from(hue, saturation * 0.95, lightness * 0.95).to_rgb();
        if new_color.get_lightness() < 20f32 || new_color.get_saturation() < 20f32 {
            self.color = Hsl::from(hue, 20f32, 20f32).to_rgb().to_css_hex_string();
        } else {
            self.color = new_color.to_css_hex_string();
        }
    }
}

fn get_random_char() -> char {
    let mut rng = rand::rng();
    let random_index = rng.random_range(0..CHARS.len());
    CHARS[random_index]
}