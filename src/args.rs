/*
 * Copyright 2021 LtPeriwinkle
 *
 * Licensed under GPLv3 or later.
 * Refer to included LICENSE file.
 */

use std::env;
use std::path::PathBuf;
use std::str::FromStr;

pub enum Action {
    Interpret,
    Compile,
    Check,
}

impl FromStr for Action {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_lowercase();
        match s.as_str() {
            "interpret" | "i" => Ok(Action::Interpret),
            "compile" | "c" => Ok(Action::Compile),
            "verify" | "check" | "v" => Ok(Action::Check),
            _ => Err("Not a valid action".into()),
        }
    }
}

pub struct Config {
    pub act: Action,
    pub infile: PathBuf,
    pub dbg: bool,
}

impl Config {
    pub fn get() -> Result<Self, String> {
        let args = env::args();
        let mut args = args.skip(1);
        let cfg = Config {
            dbg: false,
            act: Action::from_str(&args.next().ok_or("Not enough arguments.".to_owned())?)?,
            infile: PathBuf::from_str(&args.next().ok_or("Not enough arguments.".to_owned())?)
                .unwrap(),
        };
        if let Some(d) = args.next() {
            if d == "--debug" || d == "-d" {
                cfg.dbg = true;
            }
        }
        Ok(cfg)
    }
}
