use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

//    let config = parse_config(&args);
   let config = Config::build(&args).unwrap_or_else(|_err| {
  
    process::exit(1);
   });
   

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
   if let Err(_e) =  run(config) {
    
    process::exit(1);

   }

}
