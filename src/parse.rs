use std::time::Duration;
use std::path::PathBuf;
use std::fs::read_to_string;

use crate::TasError;

pub struct Tas {
    lines: Vec<Line>
}

struct Line {
    delay: Duration,
    on: Option<u16>,
    off: Option<u16>,
    sticks: Option<(Stick, Stick)>
}


struct Stick {
    angle: u16,
    magnitude: u16
}

enum Token {
    Number(u64, usize),
    Operation(String, usize),
    BracketOpen(usize),
    BracketClose(usize),
    Key(String, usize),
    Comma(String, usize),
    Newline(usize),
}

// bitflags "enum"
#[allow(dead_code)]
mod key {
    pub const NONE: u16 = 0b0;
    pub const A: u16 = 0b1;
    pub const B: u16 = 0b10;
    pub const X: u16 = 0b100;
    pub const Y: u16 = 0b1000;
    pub const L: u16 = 0b10000;
    pub const R: u16 = 0b100000;
    pub const ZL: u16 = 0b1000000;
    pub const ZR: u16 = 0b10000000;
    pub const DUP: u16 = 0b100000000;
    pub const DDOWN: u16 = 0b1000000000;
    pub const DLEFT: u16 = 0b10000000000;
    pub const DRIGHT: u16 = 0b100000000000;
    pub const PLUS: u16 = 0b1000000000000;
    pub const MINUS: u16 = 0b10000000000000;
    pub const LSTICK: u16 = 0b100000000000000;
    pub const RSTICK: u16 = 0b1000000000000000;
    pub const ALL: u16 = 0b1111111111111111;
}

fn lex(input: String) -> Result<Vec<Token>, TasError> {
    for (idx, chr) in input.char_indices().peekable() {

    }
    Ok(Vec::new())
}

pub fn parse(infile: PathBuf) -> Result<Tas, TasError> {
    let prog = read_to_string(infile).map_err(|e| TasError::Fs {e: format!("{}", e)})?;
    let tok = lex(prog);
    Ok(Tas {lines: Vec::new()})
}
