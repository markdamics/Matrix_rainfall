use anyhow::{Context, Result};
use colors_transform::{Color, Hsl, Rgb};
use crossterm::queue;
use crossterm::style::{ Print, SetForegroundColor};
use rand::Rng;
use crate::get_crossterm_color;

//const CHARS: [char; 2] = ['1', '0']; // This is working
const CHARS: [char; 66] = [
    'ﾊ', 'ﾐ', 'ﾋ', 'ｰ', 'ｳ', 'ｼ', 'ﾅ', 'ﾓ', 'ﾆ', 'ｻ', 'ﾜ', 'ﾂ', 'ｵ', 'ﾘ', 'ｱ', 'ﾎ', 'ﾃ', 'ﾏ', 'ｹ', 'ﾒ', 'ｴ', 'ｶ', 'ｷ', 'ﾑ', 'ﾕ', 'ﾗ', 'ｾ', 'ﾈ', 'ｽ', 'ﾀ', 'ﾇ', 'ﾍ', 'ｦ', 'ｲ', 'ｸ', 'ｺ', 'ｿ', 'ﾁ', 'ﾄ', 'ﾉ', 'ﾌ', 'ﾔ', 'ﾖ', 'ﾙ', 'ﾚ', 'ﾛ', 'ﾝ', '0', '1', '2', '3', '4', '5', '7', '8', '9', 'Z', '=', '*', '+', '-', '<', '>', '¦', '|', 'ç'
]; // This is not working, why?


#[derive(Clone)]
pub struct Symbol {
    character: char,
    color: String,
    base_color: String
}

impl Symbol {
    pub fn set_first_symbol(color: &str) -> Self {
        Self {
            character: get_random_char(),
            color: String::from("#FFFFFF"),
            base_color: String::from(color)
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
            base_color: String::from("#000000")
        }
    }

    pub fn init(&self, stdout: &mut std::io::Stdout) -> Result<()> {
        let foreground_color = get_crossterm_color(&self.color);
        queue!(stdout, SetForegroundColor(foreground_color))?;
        queue!(stdout, Print(self.character.to_string())).context("initialize with empty characters")?;
        Ok(())
    }

    pub fn fade(&mut self) -> () {
        //This makes the first symbol white
        if self.color == "#FFFFFF" {
            self.color = self.base_color.clone();
            return;
        }
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