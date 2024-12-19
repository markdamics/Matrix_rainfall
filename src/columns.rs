use std::io::Write;
use anyhow::Result;
use rand::Rng;

#[derive(Clone)]
pub struct Columns {
    height: u32,
}

impl Columns {
    pub fn new(height: u32) -> Self {
        Self {
            height
        }
    }

    pub fn render<W: Write>(&self, stdout: &mut W) -> Result<()> {

        Ok(())
    }

    pub fn step(&mut self) {
    }
}

fn random_chars() -> char {
    let CHARS: &str = "日ﾊﾐﾋｰｳｼﾅﾓﾆｻﾜﾂｵﾘｱﾎﾃﾏｹﾒｴｶｷﾑﾕﾗｾﾈｽﾀﾇﾍｦｲｸｺｿﾁﾄﾉﾌﾔﾖﾙﾚﾛﾝ012345789:・.=*+-<>¦｜";
    let idx = rand::thread_rng().gen_range(0..CHARS.len());
    CHARS.chars().nth(idx).unwrap()
}