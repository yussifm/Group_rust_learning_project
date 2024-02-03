use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    }

    Ok(())
}
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let to_ignore_case = args[3].clone();

        //  let ignore_case = env::var("IGNORE_CASE").is_ok();
        // IGNORE_CASE=1 cargo run -- to poem.txt
        // Powershell
        // PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt

        // This will make IGNORE_CASE persist for the remainder of your shell session. It can be unset with
        // the Remove-Item cmdlet:
        //PS> Remove-Item Env:IGNORE_CASE
        
        let mut ignore_case: bool = false;
        if to_ignore_case.trim().to_lowercase() == "y" {
            ignore_case = true;
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }
    results
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.trim());
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Pick";
        let contents = "\
        Rust
        safe, fast, productive. 
        Pick three.";

        assert_eq!(vec!["Pick three."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "SaFe";
        let contents = "\
        Rust
        safe, fast, productive. 
        Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn one_result() {
        let query = "safe";
        let contents = "\
        Rust
        safe, fast, productive. 
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
