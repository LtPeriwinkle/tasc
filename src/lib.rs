/*
 * Copyright 2021 LtPeriwinkle
 *
 * Licensed under GPLv3 or later.
 * Refer to included LICENSE file.
 */

use std::fmt::{Display, Formatter};
use std::path::PathBuf;

mod args;
pub use args::*;
mod parse;
mod vigem;

#[derive(Debug)]
pub enum TasError {
    Parse {
        l: usize,
        c: usize,
        e: &'static str,
        p: PathBuf,
    },
    Syntax {
        l: usize,
        c: usize,
        e: &'static str,
        p: PathBuf,
    },
    Fs {
        e: String,
    },
    Vigem {
        e: String,
    },
}

impl Display for TasError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let rep = match self {
            TasError::Parse { l, c, e, p } => format!(
                "\x1b[31;1mParse Error:\x1b[0m \x1b[1m{}\x1b[0m\n\t->{}:{}:{}",
                e,
                p.display(),
                l,
                c
            ),
            TasError::Syntax { l, c, e, p } => format!(
                "\x1b[31;1mSyntax Error:\x1b[0m \x1b[1m{}\x1b[0m\n\t->{}:{}:{}",
                e,
                p.display(),
                l,
                c
            ),
            TasError::Fs { e } => format!("{}", e),
            TasError::Vigem { e } => format!("ViGEm Error: {}", e),
        };
        write!(f, "{}", rep)
    }
}

pub fn run_tas(cfg: Config) -> Result<(), TasError> {
    let tas = parse::gen_tas(cfg.infile)?;
    Ok(())
}
