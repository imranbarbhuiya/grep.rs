use std::env;
use std::process;

use search_file::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = search_file::run(config) {
        eprintln!("Application error: {e}");

        process::exit(1);
    }
}
