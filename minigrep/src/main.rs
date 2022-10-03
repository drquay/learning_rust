use std::env;
use std::process;
use minigrep::Config;


// cargo run -- searchstring poem.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Proleam parsing parameters: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error {e}");
        process::exit(1);
    }
}
