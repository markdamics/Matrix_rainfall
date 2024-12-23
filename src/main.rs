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

    let mut args = std::env::args();
    let command = args.next().expect("args should have at least command");

    let mut character_color = String::from("#04ff00");
    let mut background_color = String::from("#000000");

    let mut speed: u32 = 13;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" => {
                usage(&command);
                std::process::exit(0);
            }
            "--fontcolor" => {
                let hexcolor = args.next().expect("hex color provided after --fontcolor");
                character_color = hexcolor;
            }
            "--backgroundcolor" => {
                let hexcolor = args.next().expect("hex color provided after --fontbackground");
                background_color = hexcolor;
            }
            "--speed" => {
                speed = args
                    .next()
                    .expect("speed provided after --speed")
                    .parse::<u32>()
                    .expect("specified speed is a number");
                if speed > 120 {
                    eprintln!("Speed is limited to 0-120");
                    usage(&command);
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("Unknown argument {arg}");
                usage(&command);
                std::process::exit(1);
            }
        }
    }

    let (width, height) = terminal::size().context("Get terminal size")?;

    let mut matrix = Matrix::new(width, height, &character_color, &background_color);

    let frame_speed = (1000f64 / speed as f64).round() as u64;

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

fn usage(command: &str) {
    eprintln!("Usage:");
    eprintln!("  {command} [OPTIONS]");
    eprintln!("Options:");
    eprintln!("  --fontcolor HEXCOLOR");
    eprintln!("  --backgroundcolor HEXCOLOR");
    eprintln!("  --speed UPDATES_PER_SEC");
}
