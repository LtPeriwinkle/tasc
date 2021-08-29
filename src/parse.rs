use std::fs::read_to_string;
use std::path::PathBuf;
use std::time::Duration;

use crate::TasError;

pub struct Tas {
    lines: Vec<Line>,
}

struct Line {
    delay: Duration,
    on: Option<u16>,
    off: Option<u16>,
    sticks: Option<(Stick, Stick)>,
}

struct Stick {
    angle: u16,
    magnitude: u16,
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
    let mut line = 0;
    let mut col = 0;
    while let Some(chr) = it.next() {
        match chr {
            ' ' | '\t' => out.push(Token::Whitespace((line, col))),
            '\n' => {
                out.push(Token::Newline((line, col)));
                line += 1;
                col = 0;
            }
            '{' => out.push(Token::BracketOpen((line, col))),
            '}' => out.push(Token::BracketClose((line, col))),
            ',' => out.push(Token::Comma((line, col))),
            '0'..='9' => {
                let mut num = String::from(chr);
                while let Some(d) = it.peek().filter(|c| c.is_ascii_digit()) {
                    num.push(*d);
                    it.next();
                }
                out.push(Token::Number(num.parse().unwrap(), (line, col)));
            }
            'K' | 'A' | 'N' => {
                let last_tok = &out[out.len() - 1];
                if let Token::BracketOpen(_) | Token::Comma(_) = last_tok {
                    let mut key = String::from(chr);
                    while let Some(c) = it.peek().filter(|&c| c.is_ascii_uppercase() || *c == '_') {
                        key.push(*c);
                        it.next();
                    }
                    out.push(Token::Key(key, (line, col)));
                } else {
                    return Err(TasError::Parse {
                        l: line,
                        c: col,
                        e: "Expected one of `{` or `,` before key identifier.",
                    });
                }
            }
            'O' | 'R' | 'L' => {
                let last_tok = &out[out.len() - 1];
                if let Token::Whitespace(_) = last_tok {
                    let mut op = String::from(chr);
                    while let Some(c) = it.peek().filter(|c| c.is_ascii_uppercase()) {
                        op.push(*c);
                        it.next();
                    }
                    out.push(Token::Operation(op, (line, col)));
                } else {
                    return Err(TasError::Parse {
                        l: line,
                        c: col,
                        e: "Expected whitespace before operation.",
                    });
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
    let prog = read_to_string(infile).map_err(|e| TasError::Fs {
        e: format!("{}", e),
    })?;
    let tok = lex(prog)?;
    println!("{:?}", tok);
    Ok(Tas { lines: Vec::new() })
}
