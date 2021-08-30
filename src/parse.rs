/*
 * Copyright 2021 LtPeriwinkle
 *
 * Licensed under GPLv3 or later.
 * Refer to included LICENSE file.
 */

use std::fs::read_to_string;
use std::path::PathBuf;
use std::time::Duration;
use std::slice::Iter;
use once_cell::sync::OnceCell;
use std::fmt::{Display, Formatter};

static PATH: OnceCell<PathBuf> = OnceCell::new();

use crate::TasError;

#[derive(Debug)]
pub struct Tas {
    lines: Vec<Line>,
}

impl Tas {
    fn parse_tas(prog: Vec<Token>) -> Result<Self, TasError> {
        let mut lines = vec![];
        let prog_lines = prog.split(|t| matches!(t, Token::Newline(_)));
        for line in prog_lines {
            lines.push(Line::get(line)?);
        }
        Ok(Tas {lines})
    }
}
impl Display for Tas {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "Frames   On               Off              Left Stick           Right Stick")?;
        for l in &self.lines {
            writeln!(f, "{}", l)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Line {
    delay: Duration,
    on: u16,
    off: u16,
    lstick: Option<Stick>,
    rstick: Option<Stick>
}

impl Line {
    fn new() -> Self {
        Line {
            delay: Duration::ZERO,
            on: key::NONE,
            off: key::NONE,
            lstick: None,
            rstick: None
        }
    }
    fn get(line: &[Token]) -> Result<Self, TasError> {
        let mut out = Line::new();
        let mut line = line.iter();
        while let Some(tok) = line.next() {
            match tok {
                Token::Number(n, _) => {
                    out.delay = Duration::from_nanos(16666666 * n);
                }
                Token::Operation(op, (l, c)) => {
                    match op.as_str() {
                        "ON" => {
                            line.next();
                            out.on = get_keys(&mut line)?;
                        }
                        "OFF" => {
                            line.next();
                            out.off = get_keys(&mut line)?;
                        }
                        "LSTICK" => {
                            line.next();
                            out.lstick = Some(Stick::get(&mut line)?);
                        }
                        "RSTICK" => {
                            line.next();
                            out.rstick = Some(Stick::get(&mut line)?);
                        }
                        "RAW" => {
                            line.next();
                            out.off = key::ALL;
                            out.on = get_keys(&mut line)?;
                        }
                        _ => {
                            return Err(TasError::Parse {l: *l, c: *c, e: "Unknown operation.", p: PATH.get().unwrap().into()});
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(out)
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let fr = " ".repeat(8 - format!("{}", self.delay.as_nanos() / 16666666).len());
        let o1 = " ".repeat(16 - format!("{:b}", self.on).len());
        let o2 = " ".repeat(16 - format!("{:b}", self.off).len());
        let l = " ".repeat(20 - format!("{}", self.lstick.unwrap_or(Stick::new())).len());
        write!(f, "{}{} {:b}{} {:b}{} {}{} {}", self.delay.as_nanos() / 16666666, fr, self.on, o1, self.off, o2, self.lstick.unwrap_or(Stick::new()), l, self.rstick.unwrap_or(Stick::new()))
    }
}

fn get_keys(line: &mut Iter<Token>) -> Result<u16, TasError> {
    let mut keys = key::NONE;
    for tok in line {
        if let Token::Key(k, (l, c)) =  tok {
            if let Some(n) = key2u16(k) {
                keys |= n;
            } else {
                return Err(TasError::Parse {l: *l, c: *c, e: "Unknown key identifier.", p: PATH.get().unwrap().into()});
            }
        } else if let Token::BracketClose(_) = tok {
            break;
        }
    }
    Ok(keys)
}

fn key2u16(key: &str) -> Option<u16> {
    if key.starts_with('K') { 
        let key = key.split_once('_')?.1;
        match key {
            "A" => Some(key::A),
            "B" => Some(key::B),
            "X" => Some(key::X),
            "Y" => Some(key::Y),
            "L" => Some(key::L),
            "R" => Some(key::R),
            "ZL" => Some(key::ZL),
            "ZR" => Some(key::ZR),
            "DUP" => Some(key::DUP),
            "DDOWN" => Some(key::DDOWN),
            "DLEFT" => Some(key::DLEFT),
            "DRIGHT" => Some(key::DRIGHT),
            "PLUS" => Some(key::PLUS),
            "MINUS" => Some(key::MINUS),
            "LSTICK" => Some(key::LSTICK),
            "RSTICK" => Some(key::RSTICK),
            _ => None
        }
    } else {
        match key {
            "NONE" => Some(key::NONE),
            "ALL" => Some(key::ALL),
            _ => None
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Stick {
    x: i16,
    y: i16
}

impl Stick {
    fn new() -> Self {
        Stick {
            x: 0,
            y: 0
        }
    }
    fn get(line: &mut Iter<Token>) -> Result<Self, TasError> {
        let mut stick = Stick::new();
        let mut ang: f64 = f64::NAN;
        let mut l: usize = 0;
        let mut c: usize = 0;
        if let Some(Token::Number(a, (lin, col))) = line.next() {
            l = *lin;
            c = *col;
            ang = ((*a as f64) * std::f64::consts::PI) / 180.0;
        }
        // skip comma
        line.next();
        if let Some(Token::Number(m, (l, c))) = line.next() {
            if !ang.is_nan() {
                stick.x = (ang.sin() * *m as f64).ceil() as i16;
                stick.y = (ang.cos() * *m as f64).ceil() as i16;
            } else {
                return Err(TasError::Parse {l: *l, c: *c, e: "Malformed stick information.", p: PATH.get().unwrap().into()});
            }
        } else {
            return Err(TasError::Parse {l, c, e: "Malformed stick information.", p: PATH.get().unwrap().into()});
        }
        Ok(stick)
    }
}

impl Display for Stick {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "(x: {}; y: {})", self.x, self.y)
    }
}

#[derive(Debug)]
enum Token {
    Number(u64, (usize, usize)),
    Operation(String, (usize, usize)),
    BracketOpen((usize, usize)),
    BracketClose((usize, usize)),
    Key(String, (usize, usize)),
    Comma((usize, usize)),
    Newline((usize, usize)),
    Whitespace((usize, usize)),
}

// bitflags "enum"
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
    let mut out = vec![];
    let mut it = input.chars().peekable();
    let mut line = 1;
    let mut col = 0;
    let mut bracketed = false;
    let i = PATH.get().unwrap();
    while let Some(chr) = it.next() {
        match chr {
            '+' => {
                if line != 1 || col != 0 {
                    return Err(TasError::Syntax {
                        l: line,
                        c: col,
                        e: "`+` can only appear at the start of the script",
                        p: i.into()
                    });
                }
            }
            ' ' => out.push(Token::Whitespace((line, col))),
            '\n' => {
                if bracketed {
                    return Err(TasError::Syntax {l: line, c: col, e: "Newlines cannot appear in brackets.", p: i.into()});
                } else if !matches!(it.peek(), Some('0'..='9')) && it.peek() != None {
                    return Err(TasError::Syntax {l: line, c: col, e: "A frame number must appear at the start of each line.", p: i.into()});
                }
                out.push(Token::Newline((line, col)));
                line += 1;
                col = 0;
            }
            '{' => {
                if bracketed || !matches!(out[out.len() - 1], Token::Operation(_, _)) {
                    return Err(TasError::Syntax {l: line, c: col, e: "Unexpected opening bracket.", p: i.into()});
                }
                out.push(Token::BracketOpen((line, col)));
                bracketed = true;
            }
            '}' => {
                let last_tok = &out[out.len() - 1];
                if !bracketed || (!matches!(last_tok, Token::Key(_, _)) && !(matches!(last_tok, Token::Number(_, _)) && bracketed)) {
                    return Err(TasError::Syntax {l: line, c: col, e: "Unexpected closing bracket.", p: i.into()});
                }
                out.push(Token::BracketClose((line, col)));
                bracketed = false;
            }
            ',' => {
                let last_tok = &out[out.len() - 1];
                if !matches!(last_tok, Token::Key(_, _)) && !(matches!(last_tok, Token::Number(_, _)) && bracketed) {
                    return Err(TasError::Syntax {l: line, c: col, e: "Commas can only appear inside brackets.", p: i.into()});
                }
                out.push(Token::Comma((line, col)));
            }
            '0'..='9' => {
                let last_tok = &out[out.len() - 1];
                if !bracketed {
                    if let Token::Newline(_) = last_tok {
                        let mut num = String::from(chr);
                        while let Some(d) = it.peek().filter(|c| c.is_ascii_digit()) {
                            num.push(*d);
                            it.next();
                            col += 1;
                        }
                        out.push(Token::Number(num.parse().unwrap(), (line, col)));
                    } else {
                        return Err(TasError::Syntax {
                            l: line,
                            c: col,
                            e: "Frame numbers can only appear at the start of a line.", p: i.into()
                        });
                    }
                } else {
                    if let Token::Comma(_) | Token::BracketOpen(_) = last_tok {
                        let mut num = String::from(chr);
                        while let Some(d) = it.peek().filter(|c| c.is_ascii_digit()) {
                            num.push(*d);
                            it.next();
                            col += 1;
                        }
                        out.push(Token::Number(num.parse().unwrap(), (line, col)));
                    } else {
                        return Err(TasError::Syntax {
                            l: line,
                            c: col,
                            e: "Expected one of `{` or `,` before stick parameter.", p: i.into()
                        });
                    }
                }
            }
            'K' | 'A' | 'N' => {
                let last_tok = &out[out.len() - 1];
                if let Token::BracketOpen(_) | Token::Comma(_) = last_tok {
                    let mut key = String::from(chr);
                    while let Some(c) = it.peek().filter(|&c| c.is_ascii_uppercase() || *c == '_') {
                        key.push(*c);
                        it.next();
                        col += 1;
                    }
                    out.push(Token::Key(key, (line, col)));
                } else {
                    return Err(TasError::Syntax {
                        l: line,
                        c: col,
                        e: "Expected one of `{` or `,` before key identifier.", p: i.into()
                    });
                }
            }
            'O' | 'R' | 'L' => {
                let last_tok = &out[out.len() - 1];
                if !bracketed {
                    if let Token::Whitespace(_) = last_tok {
                        let mut op = String::from(chr);
                        while let Some(c) = it.peek().filter(|c| c.is_ascii_uppercase()) {
                            op.push(*c);
                            it.next();
                            col += 1;
                        }
                        out.push(Token::Operation(op, (line, col)));
                    } else {
                        return Err(TasError::Syntax {
                            l: line,
                            c: col,
                            e: "Expected whitespace before operation.", p: i.into()
                        });
                    }
                } else {
                    return Err(TasError::Syntax {l: line, c: col, e: "Operations cannot appear inside brackets", p: i.into()});
                }
            }
            _ => {}
        }
        if chr != '\n' {
            col += 1;
        }
    }
    Ok(out)
}

pub fn gen_tas(infile: PathBuf) -> Result<Tas, TasError> {
    PATH.set(infile.clone()).unwrap();
    let prog = read_to_string(infile.clone()).map_err(|e| TasError::Fs {
        e: format!("{}", e),
    })?;
    let tok = lex(prog)?;
    Tas::parse_tas(tok)
}
