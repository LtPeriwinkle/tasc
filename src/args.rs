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
}

impl Config {
    pub fn get() -> Result<Self, String> {
        let args = env::args();
        let mut args = args.skip(1);
        let cfg = Config {
            act: Action::from_str(&args.next().ok_or("Not enough arguments.".to_owned())?)?,
            infile: PathBuf::from_str(&args.next().ok_or("Not enough arguments.".to_owned())?)
                .unwrap(),
        };
        Ok(cfg)
    }
}
