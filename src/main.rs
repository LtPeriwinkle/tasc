use tasc::Config;
const USAGE: &str = "compile or interpret tas-script\n
USAGE:
    tasc SUBCOMMAND INPUT\n
INPUT:
    The file to be compiled or interpreted.\n
SUBCOMMAND:
    interpret, i  interpret INPUT in real time
    verify, v     check that INPUT uses valid syntax
    compile, c    compile INPUT to an executable\n";

fn main() {
    let cfg = Config::get();
    if let Err(e) = cfg {
        println!("{}", e);
        println!("{}", USAGE);
        std::process::exit(1);
    }
    let cfg = cfg.unwrap();
}
