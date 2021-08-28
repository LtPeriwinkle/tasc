use std::fmt::{Display, Formatter};

mod args;
pub use args::*;
mod parse;

#[derive(Debug)]
pub enum TasError {
    Parse {l: usize, c: usize, e: &'static str},
    Fs {e: String}
}

impl Display for TasError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let rep = match self {
            TasError::Parse {l, c, e} => format!("Parsing error at line {} col {}: {}", l, c, e),
            TasError::Fs {e} => format!("{}", e)
        };
        write!(f, "{}", rep)
    }
}

pub fn run_tas(cfg: Config) -> Result<(), TasError> {
    Ok(())
}
