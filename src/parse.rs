use std::time::Duration;
use std::path::PathBuf;
use std::fs::read_to_string;
use std::str::Lines;

use crate::TasError;

pub struct Tas {
    lines: Vec<Line>
}

impl Tas {
    pub fn parse_tas(prog: Lines) -> Result<Self, TasError> {
        let mut lines: Vec<Line> = vec![];
        for line in prog {
            let line = line.split(' ');
        }
        Ok(Tas {lines})
    }
}

struct Line {
    delay: Duration,
    On: Option<Vec<Key>>,
    Off: Option<Vec<Key>>,
    Sticks: Option<(Stick, Stick)>
}
enum Key {
    A,
    B,
    X,
    Y,
    L,
    R,
    Zl,
    Zr,
    Dup,
    Ddown,
    Dleft,
    Dright,
    Plus,
    Minus,
    Lstick,
    Rstick
}
struct Stick {
    angle: u16,
    magnitude: u16
}

pub fn parse(infile: PathBuf) -> Result<Tas, TasError> {
    let prog = read_to_string(infile).map_err(|e| TasError::Fs {e: format!("{}", e)})?;
    let lines = prog.lines();
    Ok(Tas::parse_tas(lines)?)
}
