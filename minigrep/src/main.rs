use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("================================================================");
    println!(" ========== Searching for words in any Text file =========");
    println!(" ========== word file.txt case-sensitive =========");
    println!(" ========== Eg: rust rust_docs.text y =========");
    println!(" ========== Eg: RuSt rust_docs.text N =========");
     println!("================================================================");

    //    let config = parse_config(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        // cargo run -- to poem.txt > output.txt
        // cargo run > output.txt
        eprintln!("Problem parsing arguments:{err}" );
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    println!("================================================================");
}
