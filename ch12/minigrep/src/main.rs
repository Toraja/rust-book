use minigrep;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    minigrep::run(args, &mut std::io::stdout()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
}
