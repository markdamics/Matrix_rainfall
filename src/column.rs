use anyhow::Context;
use rand::Rng;
use crate::symbol::Symbol;

#[derive(Clone)]
pub struct Column {
    width: u16,
    length: u16,
    character_color: String,
    characters: Vec<Symbol>,
    vector_start_index: usize,
    vector_end_index: usize,
}

impl Column {
    pub fn new(width: u16, length: u16, character_color: &str) -> Self {
        Self {
            width,
            length,
            character_color: String::from(character_color),
            characters: vec![Symbol::set_empty(); length as usize],
            vector_start_index: 0,
            vector_end_index: 0
        }
    }

    pub fn init(&self, stdout: &mut std::io::Stdout, height_index: usize) -> anyhow::Result<()> {
        self.characters[height_index].init(stdout).context("initialize the symbols")?;
        Ok(())
    }

    pub fn update(&mut self) -> () {
        for symbol in &mut self.characters {
            symbol.fade();
        }

        let random_index = rand::rng().random_range(0..self.width);
        let rand_distance: usize = rand::rng().random_range(self.length - 20..self.length) as usize;

        for i in 0..self.characters.len() {
            if !self.characters[i].is_empty() {
                self.characters[i].set_random_symbols();
            }
        }

        if self.vector_start_index >= rand_distance || self.vector_end_index != 0 {
            self.characters[self.vector_end_index] = Symbol::set_empty();
            self.vector_end_index = (self.vector_end_index + 1) % self.characters.len();
        }

        if self.vector_start_index == 0 && random_index > 1 {
            return;
        }

        if self.vector_start_index < self.characters.len() {
            self.characters[self.vector_start_index] = Symbol::set_first_symbol(&self.character_color);
            self.vector_start_index = (self.vector_start_index + 1) % self.characters.len();
        }
    }
}

