use std::{env, error::Error, fs};


pub struct Config {
    pub word_to_find : String,
    pub file_path : String,
    pub ignore_case : bool,
}



impl Config {
   pub fn build(args : &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let word_to_find = args[1].clone();

        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        

        Ok(Config { word_to_find, file_path , ignore_case})
    }
}

pub fn run(config : Config) -> Result<() , Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_insensitive(&config.word_to_find, &contents)
    } else {
        search(&config.word_to_find, &contents) 
    };

    for line in results {
        println!("{line}");
    }

    Ok(())

}

pub fn search<'a>(word_to_find: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(word_to_find) {
            results.push(line);
        }
    }
    results
}

pub fn search_insensitive<'a>(word_to_find: &str, contents: &'a str) -> Vec<&'a str> {
    let word_to_find = word_to_find.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&word_to_find) {
            results.push(line);
        }
    }
    results
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let word_to_find = "duct";
        let contents  = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(word_to_find, contents));
    }


    #[test]
    fn case_insensitive() {
        let word_to_find = "rUsT";
        let contents  = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_insensitive(word_to_find, contents));
    }

}

